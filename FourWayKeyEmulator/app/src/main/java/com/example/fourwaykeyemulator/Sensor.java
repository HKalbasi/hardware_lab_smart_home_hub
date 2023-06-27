package com.example.fourwaykeyemulator;

public class Sensor {
    private final String type;
    private final String uri;

    public Sensor(String type, String uri) {
        this.type = type;
        this.uri = uri;
    }

    public String getType() {
        return type;
    }

    public String getUri() {
        return uri;
    }
}
