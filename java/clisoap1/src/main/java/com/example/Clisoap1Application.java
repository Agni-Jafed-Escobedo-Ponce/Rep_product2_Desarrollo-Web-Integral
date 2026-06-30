package com.example;

import jakarta.xml.ws.Service;
import jakarta.xml.ws.Dispatch;
import jakarta.xml.soap.*;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import javax.xml.namespace.QName;
import java.net.URL;
import java.util.Iterator;

@SpringBootApplication
@RestController
public class Clisoap1Application {

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
        
        URL url = new URL(WSDL_URL);
        Service servicio = Service.create(url, SERVICE_NAME);
        
        Dispatch<SOAPMessage> dispatch = servicio.createDispatch(
            PORT_NAME, 
            SOAPMessage.class, 
            Service.Mode.MESSAGE
        );

        // Crear mensaje SOAP
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
        
        // Invocar servicio
        SOAPMessage response = dispatch.invoke(request);
        
        System.out.println("Respuesta recibida, extrayendo resultado...");
        
        // Extraer resultado
        body = response.getSOAPBody();
        Iterator<?> iterator = body.getChildElements();
        
        while (iterator.hasNext()) {
            Object next = iterator.next();
            if (next instanceof SOAPBodyElement) {
                SOAPBodyElement element = (SOAPBodyElement) next;
                System.out.println("Elemento encontrado: " + element.getLocalName());
                
                // Iterar sobre los hijos del elemento
                Iterator<?> childIterator = element.getChildElements();
                while (childIterator.hasNext()) {
                    Object child = childIterator.next();
                    if (child instanceof SOAPElement) {
                        SOAPElement childElement = (SOAPElement) child;
                        String value = childElement.getValue();
                        System.out.println("Valor encontrado: " + value);
                        return value;
                    }
                }
            }
        }

        System.out.println("No se encontró resultado");
        return "Error: No se pudo obtener el resultado";
    }

    public static void main(String[] args) {
        SpringApplication.run(Clisoap1Application.class, args);
    }
}