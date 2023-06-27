package com.example.fourwaykeyemulator;

import java.io.IOException;

import fi.iki.elonen.NanoHTTPD;

public class EmulatorServer extends NanoHTTPD {
    private FirstFragment fragment;

    public EmulatorServer() throws IOException {
        super(8080);
        start(NanoHTTPD.SOCKET_READ_TIMEOUT, false);
    }

    public void setFragment(FirstFragment fragment) {
        this.fragment = fragment;
    }

    public Response serve(IHTTPSession session) {
        String response = "";
        switch (session.getUri()) {
            case "/conf":
                response = fragment.getConf(session.getRemoteIpAddress());
                break;
            case "/4way_key/key1/on":
                fragment.keyOn(1);
                break;
            case "/4way_key/key2/on":
                fragment.keyOn(2);
                break;
            case "/4way_key/key3/on":
                fragment.keyOn(3);
                break;
            case "/4way_key/key4/on":
                fragment.keyOn(4);
                break;
            case "/4way_key/key1/off":
                fragment.keyOff(1);
                break;
            case "/4way_key/key2/off":
                fragment.keyOff(2);
                break;
            case "/4way_key/key3/off":
                fragment.keyOff(3);
                break;
            case "/4way_key/key4/off":
                fragment.keyOff(4);
                break;
            case "/outlet/on":
                fragment.outletOn();
                break;
            case "/outlet/off":
                fragment.outletOff();
                break;
            case "/curtain/open":
                fragment.curtainOpen();
                break;
            case "/curtain/close":
                fragment.curtainClose();
                break;
            case "/temp":
                response += fragment.getTemp();
                break;
            case "/light":
                response += fragment.getLight();
                break;
        }
        return newFixedLengthResponse(response);
    }
}
