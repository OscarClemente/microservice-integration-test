package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
)

func homePage(w http.ResponseWriter, r *http.Request) {
	fmt.Println("Received home page request")
	fmt.Fprintf(w, "Test Go web app.")
}

func askRust(w http.ResponseWriter, r *http.Request) {
	fmt.Println("Received ask request")

	resp, err := http.Get("http://127.0.0.1:3001")
	if err != nil {
		log.Fatalln(err)
	}

	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		log.Fatalln(err)
	}

	fmt.Fprintf(w, "Test Go web app asks Rust web app: %s", body)
}

func setupRoutes() {
	http.HandleFunc("/", homePage)
	http.HandleFunc("/ask", askRust)
}

func main() {
	fmt.Println("Go web app started on port 3000")
	setupRoutes()
	http.ListenAndServe(":3000", nil)
}