// cpp/clisoap1/main.cpp
// Uso: ./clisoap1.exe  -> http://localhost:7001/?n=10
// Consume el servicio SOAP público NumberConversion

#include <iostream>
#include <string>
#include <curl/curl.h>
#include <regex>
#include "httplib.h"

size_t WriteCallback(void* contents, size_t size, size_t nmemb, std::string* userp) {
    userp->append((char*)contents, size * nmemb);
    return size * nmemb;
}

std::string soap_envelope(long n) {
    return "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n"
           "<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">\n"
           "  <soap:Body>\n"
           "    <NumberToWords xmlns=\"http://www.dataaccess.com/webservicesserver/\">\n"
           "      <ubiNum>" + std::to_string(n) + "</ubiNum>\n"
           "    </NumberToWords>\n"
           "  </soap:Body>\n"
           "</soap:Envelope>";
}

std::string extract_result(const std::string& xml) {
    std::regex pattern("<NumberToWordsResult>(.*?)</NumberToWordsResult>");
    std::smatch match;
    if (std::regex_search(xml, match, pattern)) {
        return match[1].str();
    }
    return "error";
}

std::string call_soap(long n) {
    CURL* curl = curl_easy_init();
    if (!curl) return "error";

    std::string readBuffer;
    std::string url = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";
    std::string body = soap_envelope(n);

    struct curl_slist* headers = NULL;
    headers = curl_slist_append(headers, "Content-Type: text/xml; charset=utf-8");
    headers = curl_slist_append(headers, "SOAPAction: \"NumberToWords\"");

    curl_easy_setopt(curl, CURLOPT_URL, url.c_str());
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body.c_str());
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &readBuffer);

    CURLcode res = curl_easy_perform(curl);
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);

    if (res == CURLE_OK) {
        return extract_result(readBuffer);
    }
    return "error";
}

int main() {
    curl_global_init(CURL_GLOBAL_DEFAULT);

    httplib::Server svr;

    svr.Get("/", [](const httplib::Request& req, httplib::Response& res) {
        long n = 10;
        if (req.has_param("n")) {
            n = std::stol(req.get_param_value("n"));
        }
        std::string resultado = call_soap(n);
        res.set_content(resultado, "text/plain; charset=utf-8");
    });

    std::cout << "http://localhost:7001/?n=10" << std::endl;
    svr.listen("0.0.0.0", 7001);

    curl_global_cleanup();
    return 0;
}