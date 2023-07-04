import subprocess
import time
import sys
import os
import pandas as pd


ARP_CMD = 'arp'
ARP_COLS = ['Address', 'HWtype', 'HWaddress', 'Flags Mask', 'Iface']

ip_table = set()


def get_arp_df(arp_res):
    entries = [entry.split() for entry in arp_res.split('\n')[1:]]
    return pd.DataFrame(data=entries, columns=ARP_COLS)


def on_new_device_connected(ip):
    print(f'new device connected with ip {ip}')
    ip_table.add(ip)


def on_device_disconnected(ip):
    print(f'device with ip {ip} is disconnected')


def update_devices(arp_df):
    available_ips = set(arp_df.loc[(arp_df['HWaddress'] != '(incompatible)') & (arp_df['Iface'] == 'wlan0'),'Address'])
    for ip in available_ips:
        if ip not in ip_table:
            on_new_device_connected(ip)
    disconnected_devices = []
    for ip in ip_table:
        if ip not in available_ips:
            disconnected_devices.append(ip)
            on_device_disconnected(ip)
    for ip in disconnected_devices:
        ip_table.remove(ip)
    

def check_arp_table():
    try:
        result = subprocess.check_output(ARP_CMD).decode()
        arp_df = get_arp_df(result.strip())
    except Exception as e:
        print(e)
        arp_df = pd.DataFrame(columns=ARP_COLS)
    update_devices(arp_df)


def main():
    args = sys.argv[1:]
    time_period = 2
    while(True):
        check_arp_table()
        time.sleep(time_period)

if __name__ == '__main__':
  main()
