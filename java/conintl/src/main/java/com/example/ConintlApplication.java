package com.example;

import com.ibm.icu.text.RuleBasedNumberFormat;
import com.ibm.icu.util.ULocale;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class ConintlApplication {

    @GetMapping("/")
    public String convertir(@RequestParam(defaultValue = "10") long n) {
        RuleBasedNumberFormat fmt =
            new RuleBasedNumberFormat(new ULocale("es"), RuleBasedNumberFormat.SPELLOUT);
        return fmt.format(n);
    }

    public static void main(String[] args) {
        SpringApplication.run(ConintlApplication.class, args);
    }
}