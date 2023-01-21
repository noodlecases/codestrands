import {useRouter} from "next/router";
import {useEffect, useState} from "react";
import {useAuth} from "../../../hooks/auth"
import {keycloakCallback} from "../../../api";
import {Navigate} from "react-router";

const KeycloakCallbackGet = () => {
    const router = useRouter()
    const [redirect, setRedirect] = useState<string>("/");
    const [success, setSuccess] = useState<boolean>();
    const { setIsAuthenticated } = useAuth()

    useEffect(() => {
        const execKeycloakCallback = async () => {
            const searchParams = new URLSearchParams(router.asPath.split("?")[1])
            const state = searchParams.get("state")
            const code = searchParams.get("code")

            if (code) {
                try {
                    let redirect = await keycloakCallback(code, state ?? undefined)
                    setIsAuthenticated(true)
                    setRedirect(redirect)
                } catch {
                    setSuccess(false)
                    return
                }

                setSuccess(true)
            } else {
                setSuccess(false)
            }
        }
        execKeycloakCallback()
    }, [router.pathname])
    
    if (success == undefined) {
        return // loading
    }
    
    if (redirect == "/") {
        router.push(`/?success=${success}`)
    } else if (redirect !== null) {
        router.push(redirect)
    }
    router.push(`/?success=${success}`)
}

export default KeycloakCallbackGet