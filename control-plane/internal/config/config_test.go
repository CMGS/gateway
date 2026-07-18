package config

import "testing"

func TestParseTenantTokens(t *testing.T) {
	tokens, err := parseTenantTokens(" acme=tok-a, labs=tok-b ,")
	if err != nil {
		t.Fatalf("parse tenant tokens: %v", err)
	}
	if tokens["acme"] != "tok-a" || tokens["labs"] != "tok-b" || len(tokens) != 2 {
		t.Errorf("tokens = %v, want acme/labs pair", tokens)
	}
	if empty, err := parseTenantTokens(""); err != nil || len(empty) != 0 {
		t.Errorf("empty input = %v, %v; want empty map", empty, err)
	}
	for _, bad := range []string{"acme", "=tok", "acme="} {
		if _, err := parseTenantTokens(bad); err == nil {
			t.Errorf("input %q accepted, want error", bad)
		}
	}
}
