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
        const state = btoa(JSON.stringify({redirect}));
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
        {params: {code, state}}
    );
    return resp.data.redirect;
}

////////////////////////////////////////////////
// Definitions for API GET routes begin here. //
////////////////////////////////////////////////

export type UserResponse = {
    firstName: string;
    lastName: string;
    username: string;
    bio: string;
    image: string;
    createdAt: number; // ms since epoch
    updatedAt: number; // ms since epoch
}
export const apiGetUserMe = async (): Promise<UserResponse> => {
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

export type UserSocialLinkResponse = {
    id: number,
    userId: number,
    name: string,
    url: string,
    createdAt: number, // ms since epoch
    updatedAt: number // ms since epoch
}
export const apiGetUserSocialLinkResponse = async (): Promise<UserSocialLinkResponse[]> => {
    const resp = await ApiClient.get(
        "users/@me/social-links",
    );
    return resp.data.map((x: any) => {
        return {
            id: x.data.id,
            userId: x.data.userId,
            name: x.data.skillId,
            url: x.data.url,
            createdAt: Date.parse(x.data.createdAt),
            updatedAt: Date.parse(x.data.updatedAt)
        }
    })
}

export type UserSkillResponse = {
    id: number,
    userId: number,
    skillId: number,
    createdAt: number, // ms since epoch
}
export const apiGetUserSkillMe = async (): Promise<UserSkillResponse[]> => {
    const resp = await ApiClient.get(
        "users/@me/skills/",
    );
    return resp.data.map((x: any) => {
        return {
            id: x.data.id,
            userId: x.data.userId,
            skillId: x.data.skillId,
            createdAt: Date.parse(x.data.createdAt)
        }
    })
}

export type UserInterestResponse = {
    id: number,
    userId: number,
    interestId: number,
    createdAt: number,
}
export const apiGetUserInterestMe = async (): Promise<UserInterestResponse[]> => {
    const resp = await ApiClient.get(
        "users/@me/interests/",
    );
    return resp.data.map((x: any) => {
        return {
            id: x.data.id,
            userId: x.data.userId,
            interestId: x.data.interestId,
            createdAt: Date.parse(x.data.createdAt)
        }
    })
}

export type ProjectResponse = {
    id: number,
    userId: number,
    name: string,
    description: string,
    url: string,
    image: string | null,
    createdAt: number, // ms since epoch
}
export const apiGetUserProjectMe = async (): Promise<ProjectResponse[]> => {
    const resp = await ApiClient.get(
        "users/@me/interests/",
    );
    return resp.data.map((x: any) => {
        return {
            id: x.id,
            userId: x.userId,
            name: x.name,
            description: x.description,
            url: x.url,
            image: x.image,
            createdAt: Date.parse(x.createdAt),
        }
    });
}

//////////////////////////////////////////////////
// Definitions for API PATCH routes begin here. //
//////////////////////////////////////////////////
type PatchUserRequest = {
    firstName?: string;
    lastName?: string;
    bio?: string;
}
export const apiPatchUserMe = async (data: PatchUserRequest): Promise<UserResponse> => {
    console.log(data)
    const resp = await ApiClient.patch(
        "users/@me/",
        data
    )
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