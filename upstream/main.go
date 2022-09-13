package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
)

var gldata = ""

func logWrite(write func([]byte) (int, error), body []byte) {
	_, err := write(body)
	if err != nil {
		log.Printf("Write failed: %v", err)
	}
}

func store(w http.ResponseWriter, req *http.Request) {
	defer req.Body.Close()
	data, err := io.ReadAll(req.Body)
	if err != nil {
		return
	}
	gldata += string(data) + "\n"
	logWrite(w.Write, []byte("store"))
}

func retrieve(w http.ResponseWriter, req *http.Request) {
	logWrite(w.Write, []byte(gldata))
}

func auth(w http.ResponseWriter, req *http.Request) {
	token := req.Header.Get("token")
	if token != "hello" {
		logWrite(w.Write, []byte("Unauthorized, token received: "+token))
	} else {
		logWrite(w.Write, []byte("Authorized"))
	}
}
func headers(w http.ResponseWriter, req *http.Request) {
	for name, headers := range req.Header {
		for _, h := range headers {
			fmt.Fprintf(w, "%v: %v\n", name, h)
		}
	}
}

func main() {
	http.HandleFunc("/store", store)
	http.HandleFunc("/retrieve", retrieve)
	http.HandleFunc("/auth", auth)
	log.Fatal(http.ListenAndServe(":8080", nil))
}
