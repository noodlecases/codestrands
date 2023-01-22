import axios from "axios";
import {number} from "prop-types";

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

////////////////////////////////////////////
// Definitions for API routes begin here. //
////////////////////////////////////////////

export type UserResponse = {
    firstName: string;
    lastName: string;
    username: string;
    bio: string;
    image: string;
    createdAt: number; // ms since epoch
    updatedAt: number; // ms since epoch
}
export const api_get_user_me = async (): Promise<UserResponse> => {
    const resp = await ApiClient.get(
        "users/@me/",
    );
    return {
        firstName: resp.data.firstName,
        lastName: resp.data.lastName,
        username: resp.data.username,
        bio: resp.data.bio,
        image: resp.data.image,
        createdAt: Date.parse(resp.data.createdAt),
        updatedAt: Date.parse(resp.data.updatedAt),
    };
}

export type UserSkillResponse = {
    id: number,
    userId: number,
    skillId: number,
    createdAt: number, // ms since epoch
}
export const api_get_user_skill_me = async (): Promise<UserSkillResponse> => {
    const resp = await ApiClient.get(
        "users/@me/",
    );
    return {
        id: resp.data.id,
        userId: resp.data.userId,
        skillId: resp.data.skillId,
        createdAt: Date.parse(resp.data.createdAt)
    };
}