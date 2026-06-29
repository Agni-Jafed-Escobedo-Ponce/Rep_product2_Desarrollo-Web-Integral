// golang/clisoap2/main.go
// Uso: go run main.go  -> http://localhost:8002/?n=10
// Consume el servicio SOAP y traduce al español con bregydoc/gtranslate

package main

import (
	"encoding/xml"
	"fmt"
	"log"
	"net/http"
	"strings"

	"github.com/bregydoc/gtranslate"
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

	// 1. Llamada SOAP
	soapClient, err := gosoap.SoapClient(wsdl, nil)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	res, err := soapClient.Call("NumberToWords", gosoap.Params{"ubiNum": n})
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
	en := strings.TrimSpace(response.Result)

	// 2. Traducir inglés -> español
	es, err := gtranslate.TranslateWithParams(en, gtranslate.TranslationParams{
		From: "en",
		To:   "es",
	})
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}

	fmt.Fprint(w, es)
}

func main() {
	http.HandleFunc("/", handler)
	log.Println("http://localhost:8002/?n=10")
	log.Fatal(http.ListenAndServe(":8002", nil))
}