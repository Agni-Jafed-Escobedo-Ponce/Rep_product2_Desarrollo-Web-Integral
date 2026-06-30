// cpp/conintl/main.cpp
// Uso: ./conintl.exe  -> http://localhost:7003/?n=10
// Convierte número a letras en español

#include <iostream>
#include <string>
#include <vector>
#include "httplib.h"

const std::vector<std::string> UNIDADES = {
    "", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve",
    "diez", "once", "doce", "trece", "catorce", "quince",
    "dieciséis", "diecisiete", "dieciocho", "diecinueve"
};

const std::vector<std::string> DECENAS = {
    "", "", "veinte", "treinta", "cuarenta", "cincuenta",
    "sesenta", "setenta", "ochenta", "noventa"
};

const std::vector<std::string> CENTENAS = {
    "", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos",
    "seiscientos", "setecientos", "ochocientos", "novecientos"
};

std::string convertir_es(long long n) {
    if (n == 0) return "cero";
    if (n < 0) return "menos " + convertir_es(-n);
    if (n == 100) return "cien";
    if (n < 20) return UNIDADES[n];
    if (n < 100) {
        long long d = n / 10, u = n % 10;
        if (u == 0) return DECENAS[d];
        if (d == 2) return "veinti" + UNIDADES[u];
        return DECENAS[d] + " y " + UNIDADES[u];
    }
    if (n < 1000) {
        long long c = n / 100, r = n % 100;
        if (r == 0) return CENTENAS[c];
        return CENTENAS[c] + " " + convertir_es(r);
    }
    if (n < 2000) {
        long long r = n % 1000;
        if (r == 0) return "mil";
        return "mil " + convertir_es(r);
    }
    if (n < 1000000) {
        long long m = n / 1000, r = n % 1000;
        std::string base = convertir_es(m) + " mil";
        if (r == 0) return base;
        return base + " " + convertir_es(r);
    }
    long long m = n / 1000000, r = n % 1000000;
    std::string base = (m == 1) ? "un millón" : convertir_es(m) + " millones";
    if (r == 0) return base;
    return base + " " + convertir_es(r);
}

int main() {
    httplib::Server svr;

    svr.Get("/", [](const httplib::Request& req, httplib::Response& res) {
        long long n = 10;
        if (req.has_param("n")) {
            n = std::stoll(req.get_param_value("n"));
        }
        std::string resultado = convertir_es(n);
        res.set_content(resultado, "text/plain; charset=utf-8");
    });

    std::cout << "http://localhost:7003/?n=10" << std::endl;
    svr.listen("0.0.0.0", 7003);

    return 0;
}