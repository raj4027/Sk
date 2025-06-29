package main

import (
    "html/template"
    "log"
    "net/http"
)

var templates *template.Template

func init() {
    var err error
	// Check for errors in parsing templates
	templates, err = template.ParseFiles(
		"templates/layout.html",
		"templates/index.html",
		"templates/signup.html",
		"templates/login.html",
		"templates/categories.html",
	)
	

    if err != nil {
        log.Fatal("Error loading templates:", err)
    }
}

func renderTemplate(w http.ResponseWriter, contentTemplate string, data interface{}) {
    files := []string{
        "templates/layout.html",
        "templates/" + contentTemplate,
    }

    tmpl, err := template.ParseFiles(files...)
    if err != nil {
        http.Error(w, "Template parsing error: "+err.Error(), http.StatusInternalServerError)
        return
    }

    err = tmpl.ExecuteTemplate(w, "layout", data)
    if err != nil {
        http.Error(w, "Template execution error: "+err.Error(), http.StatusInternalServerError)
    }
}

func main() {
    // Static file handling
    fs := http.FileServer(http.Dir("static"))
    http.Handle("/static/", http.StripPrefix("/static/", fs))

    // Routes
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        renderTemplate(w, "index.html", nil)
    })

    http.HandleFunc("/signup", func(w http.ResponseWriter, r *http.Request) {
        renderTemplate(w, "signup.html", nil)
    })

    http.HandleFunc("/login", func(w http.ResponseWriter, r *http.Request) {
        renderTemplate(w, "login.html", nil)
    })

    http.HandleFunc("/categories", func(w http.ResponseWriter, r *http.Request) {
        renderTemplate(w, "categories.html", nil)
    })

    log.Println("Server started at http://localhost:8080")
    log.Fatal(http.ListenAndServe(":8080", nil))
}