package main

import (
	"fmt"
	"os"
	"os/exec"
	"log"
	"strings"
	"strconv"
	"regexp"
	"net/http"
)

var RestrictIp = ""
const ListenIp = "0.0.0.0"
var ListenPort = 5001
var BrowserCmd = "open"

func host() string {
	return ListenIp + ":" + fmt.Sprint(ListenPort)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func respond(w http.ResponseWriter, msg string, msgInfo string, status int) {
	w.Header().Add("Access-Control-Allow-Origin", "*")
	w.WriteHeader(status)
	if len(msgInfo) == 0 {
		log.Printf(msg)
		fmt.Fprintf(w, msg)
	} else {
		log.Printf(msg, msgInfo)
		fmt.Fprintf(w, msg, msgInfo)
	}
}

func get_bookmarklet(w http.ResponseWriter, r *http.Request) {
	bookmarklet, err := os.ReadFile("bookmarklet.js")
	check(err)
	html, err := os.ReadFile("bookmarklet.html")
	check(err)
	htmlStr := strings.Replace(string(html[:]), "REPLACE_JS", string(bookmarklet[:]), 1)
	fmt.Fprintf(w, htmlStr)
}

func open_url(w http.ResponseWriter, r *http.Request) {
	url := r.URL.Query().Get("url")
	if url == "" {
		respond(w, "No `url` parameter given.", "", http.StatusBadRequest)
		return
	}

	match, _ := regexp.Match("^https?://.*", []byte(url))
	if !match {
		respond(w, "Invalid URL: %s", url, http.StatusBadRequest)
		return
	}

	remoteIp := strings.Split(r.RemoteAddr, ":")[0]
	if RestrictIp != "" {
		if remoteIp != RestrictIp {
			respond(w, "Invalid client IP: %s", r.RemoteAddr, http.StatusForbidden)
			return
		}
	}

	cmd := exec.Command(BrowserCmd, url)
	err := cmd.Run()
	if err != nil {
		respond(w, "Failed to open URL %s on remote machine", url, http.StatusInternalServerError)
		return
	}
	notif := exec.Command(
		"notify-send",
		"Listener opened a URL",
		"<a href=\"" + url + "\">" + url + "</a>",
	)
	notif.Run()

	log.Printf("Opened URL from %s → %s", remoteIp, url)
	fmt.Fprintf(w, "Opened URL %s", url)
}

func main() {

	if len(os.Args) < 3 {
		log.Fatal("Usage:\n\n  listener <listen port> <browser command> [restrict to IP]\n\nPassing an IP address as the lasp parameter causes listener to reject any requests NOT from that IP.\n\nmacOS example:\n    listener 5000 open\n\nLinux example:\n    listener 6969 /usr/bin/firefox 142.250.179.174")
	}
	if len(os.Args) == 4 {
		RestrictIp = os.Args[3]
	}
	lp, err := strconv.Atoi(os.Args[1])
	if err != nil {
		log.Fatal("Invalid port number provided")
	}
	ListenPort = lp
	BrowserCmd = os.Args[2]
	http.HandleFunc("/bookmark", get_bookmarklet);
	http.HandleFunc("/", open_url);
	log.Printf("Listening on %s", host())
	log.Fatal(http.ListenAndServe(host(), nil))
}
