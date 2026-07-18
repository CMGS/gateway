package httpapi

import (
	"os"
	"regexp"
	"strings"
	"testing"
)

var (
	routePattern  = regexp.MustCompile(`"(GET|POST|PUT|PATCH|DELETE) /api/v1(/[^"]*)"`)
	methodPattern = regexp.MustCompile(`^    (get|post|put|patch|delete):`)
)

func TestRoutesMatchOpenAPISpec(t *testing.T) {
	src, err := os.ReadFile("server.go")
	if err != nil {
		t.Fatalf("read server.go: %v", err)
	}
	served := make(map[string]bool)
	for _, m := range routePattern.FindAllStringSubmatch(string(src), -1) {
		served[strings.ToLower(m[1])+" "+m[2]] = true
	}
	if len(served) == 0 {
		t.Fatal("no routes found in server.go")
	}

	spec, err := os.ReadFile("../../api/openapi.yaml")
	if err != nil {
		t.Fatalf("read openapi.yaml: %v", err)
	}
	declared := make(map[string]bool)
	inPaths := false
	path := ""
	for line := range strings.SplitSeq(string(spec), "\n") {
		switch {
		case line == "paths:":
			inPaths = true
		case inPaths && line != "" && !strings.HasPrefix(line, " "):
			inPaths = false
		case inPaths && strings.HasPrefix(line, "  /"):
			path = strings.TrimSuffix(strings.TrimSpace(line), ":")
		case inPaths && methodPattern.MatchString(line):
			method := strings.TrimSuffix(strings.TrimSpace(line), ":")
			declared[method+" "+path] = true
		}
	}
	if len(declared) == 0 {
		t.Fatal("no paths found in openapi.yaml")
	}

	for route := range served {
		if !declared[route] {
			t.Errorf("route %q served but missing from api/openapi.yaml", route)
		}
	}
	for route := range declared {
		if !served[route] {
			t.Errorf("path %q declared in api/openapi.yaml but not served", route)
		}
	}
}
