package com.example.fourwaykeyemulator;

import java.util.ArrayList;

public class Room {
    private final String room;
    private final ArrayList<Sensor> sensors;
    private final ArrayList<Sensor> actuators;

    public Room(String room, ArrayList<Sensor> sensors, ArrayList<Sensor> actuators) {
        this.room = room;
        this.sensors = sensors;
        this.actuators = actuators;
    }

    public String getRoom() {
        return room;
    }

    public ArrayList<Sensor> getSensors() {
        return sensors;
    }

    public ArrayList<Sensor> getActuators() {
        return actuators;
    }
}
