package com.example.fourwaykeyemulator;

import android.annotation.SuppressLint;
import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;

import androidx.annotation.NonNull;
import androidx.fragment.app.Fragment;

import com.example.fourwaykeyemulator.databinding.FragmentFirstBinding;
import com.google.gson.Gson;

import java.io.IOException;
import java.util.ArrayList;

import okhttp3.Call;
import okhttp3.Callback;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.Response;

public class FirstFragment extends Fragment {

    private FragmentFirstBinding binding;
    private int temp;
    private int light;
    private String room;
    private OkHttpClient client;
    private String hubIp;
    private Gson gson;

    @SuppressLint("SetTextI18n")
    @Override
    public View onCreateView(
            @NonNull LayoutInflater inflater, ViewGroup container,
            Bundle savedInstanceState
    ) {

        binding = FragmentFirstBinding.inflate(inflater, container, false);
        MainActivity.getServer().setFragment(this);

        gson = new Gson();

        room = "حال";

        temp = 25;
        light = 200;

        client = new OkHttpClient();

        binding.roomButton.setOnClickListener(view -> {
            if (!binding.roomEditText.getText().toString().isEmpty()) {
                room = binding.roomEditText.getText().toString();
                binding.roomEditText.setText("");
                binding.roomTextView.setText(room);
            }
        });

        binding.tempButton.setOnClickListener(view -> {
            if (!binding.tempEditText.getText().toString().isEmpty()) {
                temp = Integer.parseInt(binding.tempEditText.getText().toString());
                binding.tempEditText.setText("");
                binding.tempTextView.setText("" + temp);
            }
        });

        binding.lightButton.setOnClickListener(view -> {
            if (!binding.lightEditText.getText().toString().isEmpty()) {
                light = Integer.parseInt(binding.lightEditText.getText().toString());
                binding.lightEditText.setText("");
                binding.lightTextView.setText("" + light);
            }
        });

        binding.moveButton.setOnClickListener(view -> {
            if (hubIp != null)
                client.newCall(new Request.Builder()
                        .url(hubIp + "/move").build()).enqueue(new Callback() {
                    @Override
                    public void onFailure(@NonNull Call call, @NonNull IOException e) {
                    }

                    @Override
                    public void onResponse(@NonNull Call call, @NonNull Response response) {
                    }
                });
        });

        return binding.getRoot();
    }

    public void onViewCreated(@NonNull View view, Bundle savedInstanceState) {
        super.onViewCreated(view, savedInstanceState);
    }

    public String getConf(String ip) {
        hubIp = "http://" + ip + ":8080";

        ArrayList<Sensor> s = new ArrayList<>();

        if (binding.tempSwitch.isChecked()) {
            s.add(new Sensor("temp", "/temp"));
        }
        if (binding.lightSwitch.isChecked()) {
            s.add(new Sensor("light", "/light"));
        }
        if (binding.moveSwitch.isChecked()) {
            s.add(new Sensor("move", "/move"));
        }

        ArrayList<Sensor> a = new ArrayList<>();

        if (binding.keySwitch.isChecked()) {
            a.add(new Sensor("4 way key", "/4way_key"));
        }
        if (binding.outletSwitch.isChecked()) {
            a.add(new Sensor("outlet", "/outlet"));
        }
        if (binding.curtainSwitch.isChecked()) {
            a.add(new Sensor("curtain", "/curtain"));
        }

        return gson.toJson(new Room(room, s, a));

//        return "{\"room\":\"" + room + "\", " +
//                "\"sensors\":" +
//                "[{\"type\":\"temp\", \"uri\":\"/temp\"}, " +
//                "{\"type\":\"light\", \"uri\":\"/light\"}, " +
//                "{\"type\":\"move\", \"uri\":\"/move\"}], " +
//                "\"actuators\":" +
//                "[{\"type\":\"4 way key\", \"uri\":\"/4way_key\"}, " +
//                "{\"type\":\"outlet\", \"uri\":\"/outlet\"}, " +
//                "{\"type\":\"curtain\", \"uri\":\"/curtain\"}]" +
//                "}";
    }

    public int getTemp() {
        return temp;
    }

    public int getLight() {
        return light;
    }

    public void keyOn(int keyNum) {
        requireActivity().runOnUiThread(() -> {
            switch (keyNum) {
                case 1:
                    binding.key1.setVisibility(View.INVISIBLE);
                    binding.key12.setVisibility(View.VISIBLE);
                    break;
                case 2:
                    binding.key2.setVisibility(View.INVISIBLE);
                    binding.key22.setVisibility(View.VISIBLE);
                    break;
                case 3:
                    binding.key3.setVisibility(View.INVISIBLE);
                    binding.key32.setVisibility(View.VISIBLE);
                    break;
                case 4:
                    binding.key4.setVisibility(View.INVISIBLE);
                    binding.key42.setVisibility(View.VISIBLE);
                    break;
            }
        });
    }

    public void keyOff(int keyNum) {
        requireActivity().runOnUiThread(() -> {
            switch (keyNum) {
                case 1:
                    binding.key12.setVisibility(View.INVISIBLE);
                    binding.key1.setVisibility(View.VISIBLE);
                    break;
                case 2:
                    binding.key22.setVisibility(View.INVISIBLE);
                    binding.key2.setVisibility(View.VISIBLE);
                    break;
                case 3:
                    binding.key32.setVisibility(View.INVISIBLE);
                    binding.key3.setVisibility(View.VISIBLE);
                    break;
                case 4:
                    binding.key42.setVisibility(View.INVISIBLE);
                    binding.key4.setVisibility(View.VISIBLE);
                    break;
            }
        });
    }

    public void outletOn() {
        requireActivity().runOnUiThread(() -> {
            binding.outlet.setVisibility(View.INVISIBLE);
            binding.outlet2.setVisibility(View.VISIBLE);
        });
    }

    public void outletOff() {
        requireActivity().runOnUiThread(() -> {
            binding.outlet2.setVisibility(View.INVISIBLE);
            binding.outlet.setVisibility(View.VISIBLE);
        });
    }

    public void curtainOpen() {
        requireActivity().runOnUiThread(() -> {
            binding.curtain.setVisibility(View.INVISIBLE);
            binding.curtain2.setVisibility(View.VISIBLE);
        });
    }

    public void curtainClose() {
        requireActivity().runOnUiThread(() -> {
            binding.curtain2.setVisibility(View.INVISIBLE);
            binding.curtain.setVisibility(View.VISIBLE);
        });
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        binding = null;
    }
}