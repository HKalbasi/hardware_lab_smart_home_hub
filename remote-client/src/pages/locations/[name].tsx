// @ts-nocheck

import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import styles from '@/styles/Home.module.css'
import { useEffect, useState } from 'react'
import { useRouter } from 'next/router'
import { subscribe } from '../api/websocket'
import { Pane, Split } from '..'

export default function Home() {
    const router = useRouter();
    const { name } = router.query;
    const [state, setState]: [any, any] = useState(undefined);
    useEffect(() => {
        subscribe((x: any) => setState(x));
    });
    const isOk = state && state.devices && (name in state.devices);
    return <>
        <Head>
            <title>هاب خانه هوشمند - {name}</title>
            <meta name="description" content="Generated by create next app" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <link rel="icon" href="/favicon.ico" />
        </Head>
        <div>
            <div style={{ display: 'flex', padding: '1rem', height: '100vh', width: '100vw' }}>
            {!state && <Pane text='ما متصل نیستیم. چند لحظه صبر کنید.' />}
            {!isOk && <Pane text={`مکان با نام ${name} وجود ندارد یا قطع شده است.`} />}
            {isOk && <Split direction='column'>
                <Pane text={name} />
                <Split direction='row'>
                    {state.devices[name].map((x) => (
                        <Pane key={x.type} text={`${x.type}: ${x.value}`}/>
                    ))}
                </Split>
            </Split>}
            </div>  
        </div>
    </>
}
