package httpapi

import (
	"net/http"

	"github.com/cocoonstack/gateway/control-plane/internal/gateway"
	"github.com/cocoonstack/gateway/control-plane/internal/user"
)

func (s *Server) overview(w http.ResponseWriter, r *http.Request) {
	since, until, bucket, err := period(r)
	if err != nil {
		writeError(w, http.StatusBadRequest, err.Error())
		return
	}
	p := current(r)
	scope := scopeFor(p.User)
	usage, err := s.gateway.Usage(r.Context(), scope, since, until)
	if err != nil {
		mapError(w, err)
		return
	}
	series, err := s.gateway.UsageSeries(r.Context(), scope, bucket, since, until)
	if err != nil {
		mapError(w, err)
		return
	}
	models, err := s.gateway.Models(r.Context(), scope)
	if err != nil {
		mapError(w, err)
		return
	}
	stripVendorCost(p.User.Role, usage, series.Points)
	var totals struct {
		Requests         int64 `json:"requests"`
		TotalTokens      int64 `json:"total_tokens"`
		CostMicros       int64 `json:"cost_micros"`
		VendorCostMicros int64 `json:"vendor_cost_micros,omitempty"`
	}
	for _, row := range usage {
		totals.Requests += row.Requests
		totals.TotalTokens += row.TotalTokens
		totals.CostMicros += row.CostMicros
		totals.VendorCostMicros += row.VendorCostMicros
	}
	writeJSON(w, http.StatusOK, map[string]any{
		"totals": totals, "usage": usage, "series": series, "models": models,
	})
}

func (s *Server) usage(w http.ResponseWriter, r *http.Request) {
	since, until, _, err := period(r)
	if err != nil {
		writeError(w, http.StatusBadRequest, err.Error())
		return
	}
	p := current(r)
	rows, err := s.gateway.Usage(r.Context(), scopeFor(p.User), since, until)
	if err != nil {
		mapError(w, err)
		return
	}
	stripVendorCost(p.User.Role, rows, nil)
	writeJSON(w, http.StatusOK, map[string]any{"usage": rows, "since": since, "until": until})
}

func (s *Server) usageSeries(w http.ResponseWriter, r *http.Request) {
	since, until, bucket, err := period(r)
	if err != nil {
		writeError(w, http.StatusBadRequest, err.Error())
		return
	}
	p := current(r)
	series, err := s.gateway.UsageSeries(r.Context(), scopeFor(p.User), bucket, since, until)
	if err != nil {
		mapError(w, err)
		return
	}
	stripVendorCost(p.User.Role, nil, series.Points)
	writeJSON(w, http.StatusOK, series)
}

func (s *Server) models(w http.ResponseWriter, r *http.Request) {
	models, err := s.gateway.Models(r.Context(), scopeFor(current(r).User))
	if err != nil {
		mapError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, map[string]any{"models": models})
}

func (s *Server) instances(w http.ResponseWriter, r *http.Request) {
	instances, err := s.gateway.Instances(r.Context())
	if err != nil {
		mapError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, map[string]any{"instances": instances})
}

func stripVendorCost(role user.Role, usage []gateway.UsageRow, points []gateway.SeriesPoint) {
	if role == user.RoleSystemAdmin {
		return
	}
	for idx := range usage {
		usage[idx].VendorCostMicros = 0
	}
	for idx := range points {
		points[idx].VendorCostMicros = 0
	}
}
