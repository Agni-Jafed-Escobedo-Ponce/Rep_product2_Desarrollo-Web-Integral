package com.example;

import jakarta.xml.ws.Service;
import jakarta.xml.ws.Dispatch;
import jakarta.xml.soap.*;
import org.json.JSONObject;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import javax.xml.namespace.QName;
import java.net.URI;
import java.net.URL;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.Iterator;

@SpringBootApplication
@RestController
public class Clisoap2Application {

    private static final String WSDL_URL =
        "https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL";

    private static final QName SERVICE_NAME = new QName(
        "http://www.dataaccess.com/webservicesserver/",
        "NumberConversion"
    );

    private static final QName PORT_NAME = new QName(
        "http://www.dataaccess.com/webservicesserver/",
        "NumberConversionSoap"
    );

    @GetMapping("/")
    public String convertir(@RequestParam(defaultValue = "10") long n) throws Exception {
        System.out.println("=== Iniciando conversión para n=" + n + " ===");
        
        // 1. Llamada SOAP con Dispatch
        URL url = new URL(WSDL_URL);
        Service servicio = Service.create(url, SERVICE_NAME);
        
        Dispatch<SOAPMessage> dispatch = servicio.createDispatch(
            PORT_NAME, 
            SOAPMessage.class, 
            Service.Mode.MESSAGE
        );

        MessageFactory messageFactory = MessageFactory.newInstance();
        SOAPMessage request = messageFactory.createMessage();
        SOAPPart soapPart = request.getSOAPPart();
        SOAPEnvelope envelope = soapPart.getEnvelope();
        SOAPBody body = envelope.getBody();

        QName operationName = new QName(
            "http://www.dataaccess.com/webservicesserver/",
            "NumberToWords"
        );
        SOAPBodyElement bodyElement = body.addBodyElement(operationName);
        SOAPElement ubiNum = bodyElement.addChildElement("ubiNum");
        ubiNum.addTextNode(String.valueOf(n));

        System.out.println("Enviando request SOAP...");
        
        SOAPMessage response = dispatch.invoke(request);
        
        System.out.println("Respuesta recibida, extrayendo resultado...");
        
        body = response.getSOAPBody();
        String textoIngles = "";
        Iterator<?> iterator = body.getChildElements();
        
        while (iterator.hasNext()) {
            Object next = iterator.next();
            if (next instanceof SOAPBodyElement) {
                SOAPBodyElement element = (SOAPBodyElement) next;
                Iterator<?> childIterator = element.getChildElements();
                while (childIterator.hasNext()) {
                    Object child = childIterator.next();
                    if (child instanceof SOAPElement) {
                        SOAPElement childElement = (SOAPElement) child;
                        textoIngles = childElement.getValue();
                        System.out.println("Texto en inglés: " + textoIngles);
                        break;
                    }
                }
            }
        }

        if (textoIngles.isEmpty()) {
            return "Error: No se pudo obtener el texto en inglés";
        }

        // 2. Traducción con MyMemory API
        System.out.println("Traduciendo a español...");
        HttpClient client = HttpClient.newHttpClient();

        // Codificar ambos parámetros correctamente
        String textoCodificado = java.net.URLEncoder.encode(textoIngles, "UTF-8");
        String langpairCodificado = java.net.URLEncoder.encode("en|es", "UTF-8");

        String urlTraduccion = "https://api.mymemory.translated.net/get?q=" + 
            textoCodificado + "&langpair=" + langpairCodificado;

        System.out.println("URL de traducción: " + urlTraduccion);

        HttpRequest httpRequest = HttpRequest.newBuilder()
            .uri(URI.create(urlTraduccion))
            .build();

        HttpResponse<String> httpResponse = client.send(httpRequest, HttpResponse.BodyHandlers.ofString());
        System.out.println("Respuesta de MyMemory: " + httpResponse.body());

        JSONObject json = new JSONObject(httpResponse.body());
        String textoEspanol = json.getJSONObject("responseData").getString("translatedText");

        System.out.println("Texto en español: " + textoEspanol);
        return textoEspanol;
    }

    public static void main(String[] args) {
        SpringApplication.run(Clisoap2Application.class, args);
    }
}