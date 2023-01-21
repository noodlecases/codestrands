import axios from "axios";

export const API_BASE_URL = "/api/v1";

const ApiClient = axios.create({
    baseURL: API_BASE_URL,
    withCredentials: true,
});

ApiClient.interceptors.response.use(
    (response) => response,
    async (error) => {
        if (error.response.status === 401) {
            let redirect = await loginRedirect(window.location.pathname);
            if (redirect) {
                if (redirect.startsWith("/")) {
                    window.location.pathname = redirect;
                } else {
                    window.location.href = redirect;
                }
            } else {
                window.location.pathname = "/?success=true";
            }
        }
        return Promise.reject(error);
    }
);

export const loginRedirect = async (
    redirect?: string
): Promise<string | null> => {
    const oauthUrl = new URL(API_BASE_URL + "/auth/login/keycloak/");

    if (redirect) {
        const state = btoa(JSON.stringify({ redirect }));
        oauthUrl.searchParams.set("state", state);
    }

    let resp = await ApiClient.get(oauthUrl.toString());
    return resp.data.redirect;
};

export async function keycloakCallback(
    code: string,
    state?: string
): Promise<string> {
    const resp = await ApiClient.post(
        "auth/callback/keycloak/",
        {},
        { params: { code, state } }
    );
    return resp.data.redirect;
}

