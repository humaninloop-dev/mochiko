package notify

import (
	"context"
	"log"
	"net/http"
	"net/url"
	"strings"
	"time"
)

// Twilio sends one SMS per call. Numbers are masked before any log line (GI-004).
type Twilio struct {
	sid, token, from string
	http             *http.Client
}

func NewTwilio(sid, token, from string) *Twilio {
	return &Twilio{sid: sid, token: token, from: from, http: &http.Client{Timeout: 10 * time.Second}}
}

func (t *Twilio) Send(ctx context.Context, to, body string) (string, error) {
	form := url.Values{"To": {to}, "From": {t.from}, "Body": {body}}
	req, _ := http.NewRequestWithContext(ctx, "POST",
		"https://api.twilio.com/2010-04-01/Accounts/"+t.sid+"/Messages.json",
		strings.NewReader(form.Encode()))
	req.SetBasicAuth(t.sid, t.token)
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	res, err := t.http.Do(req)
	if err != nil {
		log.Printf("twilio: send to %s failed: %v", mask(to), err)
		return "", err
	}
	defer res.Body.Close()
	if res.StatusCode >= 300 {
		log.Printf("twilio: send to %s status %d", mask(to), res.StatusCode)
		return "", errStatus(res.StatusCode)
	}
	return sidFrom(res.Body), nil
}

func mask(e164 string) string {
	if len(e164) < 4 {
		return "***"
	}
	return "***" + e164[len(e164)-3:]
}
