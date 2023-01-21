import "../styles/globals.css";
import type {AppProps} from "next/app";
import { AuthProvider } from "../hooks/auth";
import {useEffect} from "react";

function MyApp({Component, ...pageProps}: AppProps) {
    let isAuthenticated = false

    useEffect(() => {
        isAuthenticated = localStorage.getItem("isAuthenticated") === "true";
    }, [])

    return (
        <AuthProvider isAuthenticated={isAuthenticated}>
            <Component {...pageProps} />
        </AuthProvider>
    )

}

export default MyApp;
