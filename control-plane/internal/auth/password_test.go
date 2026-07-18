package auth

import "testing"

func TestPasswordRoundTrip(t *testing.T) {
	hash, err := HashPassword("correct-horse")
	if err != nil {
		t.Fatalf("hash password: %v", err)
	}
	if !VerifyPassword(hash, "correct-horse") {
		t.Fatal("correct password did not verify")
	}
	if VerifyPassword(hash, "wrong-password") {
		t.Fatal("wrong password verified")
	}
	if VerifyPassword("not-a-hash", "correct-horse") {
		t.Fatal("malformed hash verified")
	}
}

func TestPasswordMinimum(t *testing.T) {
	if _, err := HashPassword("short"); err == nil {
		t.Fatal("short password accepted")
	}
}
