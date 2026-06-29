// golang/clisoap1/main.go
// Uso: go run main.go  -> http://localhost:8001/?n=10
// Consume el servicio SOAP público NumberConversion

package main

import (
	"encoding/xml"
	"fmt"
	"log"
	"net/http"
	"strings"

	"github.com/tiaguinho/gosoap"
)

const wsdl = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL"

// Estructura para parsear la respuesta SOAP
type NumberToWordsResponse struct {
	XMLName xml.Name `xml:"NumberToWordsResponse"`
	Result  string   `xml:"NumberToWordsResult"`
}

func handler(w http.ResponseWriter, r *http.Request) {
	n := r.URL.Query().Get("n")
	if n == "" {
		n = "10"
	}

	soap, err := gosoap.SoapClient(wsdl, nil)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}

	params := gosoap.Params{"ubiNum": n}
	res, err := soap.Call("NumberToWords", params)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}

	// Parsear el XML para extraer el resultado
	var response NumberToWordsResponse
	err = xml.Unmarshal(res.Body, &response)
	if err != nil {
		http.Error(w, "Error al parsear respuesta: "+err.Error(), 500)
		return
	}

	resultado := strings.TrimSpace(response.Result)
	fmt.Fprint(w, resultado)
}

func main() {
	http.HandleFunc("/", handler)
	log.Println("http://localhost:8001/?n=10")
	log.Fatal(http.ListenAndServe(":8001", nil))
}