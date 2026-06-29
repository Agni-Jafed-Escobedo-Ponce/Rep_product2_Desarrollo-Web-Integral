// golang/conintl/main.go
// Uso: go run main.go  -> http://localhost:8003/?n=10
// Convierte un número a letras en español (implementación propia)

package main

import (
	"fmt"
	"log"
	"net/http"
	"strconv"
)

func handler(w http.ResponseWriter, r *http.Request) {
	nStr := r.URL.Query().Get("n")
	if nStr == "" {
		nStr = "10"
	}
	n, err := strconv.ParseInt(nStr, 10, 64)
	if err != nil {
		http.Error(w, "Número inválido", 400)
		return
	}

	texto := numeroALetrasES(n)
	fmt.Fprint(w, texto)
}

// numeroALetrasES convierte un entero a letras en español
func numeroALetrasES(n int64) string {
	return convertirES(n)
}

var unidades = []string{"", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve",
	"diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve"}
var decenas = []string{"", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"}
var centenas = []string{"", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos",
	"seiscientos", "setecientos", "ochocientos", "novecientos"}

func convertirES(n int64) string {
	if n == 0 {
		return "cero"
	}
	if n < 0 {
		return "menos " + convertirES(-n)
	}
	if n == 100 {
		return "cien"
	}
	if n < 20 {
		return unidades[n]
	}
	if n < 100 {
		d, u := n/10, n%10
		if u == 0 {
			return decenas[d]
		}
		if d == 2 {
			return "veinti" + unidades[u]
		}
		return decenas[d] + " y " + unidades[u]
	}
	if n < 1000 {
		c, r := n/100, n%100
		if r == 0 {
			return centenas[c]
		}
		return centenas[c] + " " + convertirES(r)
	}
	if n < 2000 {
		r := n % 1000
		if r == 0 {
			return "mil"
		}
		return "mil " + convertirES(r)
	}
	if n < 1_000_000 {
		m, r := n/1000, n%1000
		base := convertirES(m) + " mil"
		if r == 0 {
			return base
		}
		return base + " " + convertirES(r)
	}
	if n < 2_000_000 {
		r := n % 1_000_000
		if r == 0 {
			return "un millón"
		}
		return "un millón " + convertirES(r)
	}
	m, r := n/1_000_000, n%1_000_000
	base := convertirES(m) + " millones"
	if r == 0 {
		return base
	}
	return base + " " + convertirES(r)
}

func main() {
	http.HandleFunc("/", handler)
	log.Println("http://localhost:8003/?n=10")
	log.Fatal(http.ListenAndServe(":8003", nil))
}