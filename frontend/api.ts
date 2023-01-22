import axios, {AxiosResponse} from "axios";
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

////////////////////////////////////////////////
// Definitions for API GET routes begin here. //
////////////////////////////////////////////////

export type UserResponse = {
  firstName: string;
  lastName: string;
  username: string;
  bio?: string;
  image?: string;
  createdAt: number; // ms since epoch
  updatedAt: number; // ms since epoch
};
export const apiGetUserMe = async (): Promise<UserResponse> => {
  const resp = await ApiClient.get("users/@me/");
  return {
    firstName: resp.data.firstName,
    lastName: resp.data.lastName,
    username: resp.data.username,
    bio: resp.data.bio,
    image: resp.data.image,
    createdAt: Date.parse(resp.data.createdAt),
    updatedAt: Date.parse(resp.data.updatedAt),
  };
};

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
  id: number;
  userId: number;
  skillId: number;
  createdAt: number; // ms since epoch
};
export const apiGetUserSkillMe = async (): Promise<UserSkillResponse[]> => {
    const resp = await ApiClient.get(
        "users/@me/skills/",
    );
    return resp.data.map((x: any) => {
        return {
            id: x.id,
            userId: x.userId,
            skillId: x.skillId,
            createdAt: Date.parse(x.createdAt)
        }
    })
}

export type TopSwipeResponse = {
  first_name: string,
  last_name: string,
  username: string,
  bio: string,
  image: string,
  created_at: number,
  updated_at: number,
};
export const apiGetTopSwipeMe = async (): Promise<TopSwipeResponse[]> => {
    const resp = await ApiClient.get(
        "/users/@me/swipes/",
    );
    return resp.data.map((x: any) => {
        return {
          first_name: x.first_name,
          last_name: x.last_name,
          username: x.username,
          bio: x.bio,
          image: x.image,
          created_at: Date.parse(x.data.createdAt),
          updated_at: Date.parse(x.data.updatedAt)
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
            id: x.id,
            userId: x.userId,
            interestId: x.interestId,
            createdAt: Date.parse(x.createdAt)
        }
    })
}

export type ProjectResponse = {
    id: number,
    userId: number,
    name: string,
    description: string,
    url: string,
    image?: string,
    createdAt: number, // ms since epoch
}
export const apiGetUserProjectMe = async (): Promise<ProjectResponse[]> => {
    const resp = await ApiClient.get(
        "users/@me/projects/",
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

export type Chat = {
  id: number;
  title: string;
  createdAt: Date;
  updatedAt: Date;
};
export const getChats = async (): Promise<Chat[]> => {
  const res = await ApiClient.get("chats/");

  return res.data.map((c: any) => ({
    createdAt: Date.parse(c.createdAt),
    updatedAt: Date.parse(c.updatedAt),
    ...c,
  })) as Chat[];
};

export type WsEvent = {
  type: string;
  payload: any;
};
export type Message = {
  user_id: number;
  timestamp: Date;
  chat_id: number;
  content: string;
};

export type User = {
  id: number;
  firstName: string;
  lastName: string;
  username: string;
  bio?: string;
  image?: string;
  createdAt: number;
  updatedAt: number;
};
export const getUser = async (user_id: number): Promise<User> => {
  const res = await ApiClient.get(`users/${user_id}/`);
  res.data.createdAt = Date.parse(res.data.createdAt);
  res.data.updatedAt = Date.parse(res.data.updatedAt);
  return res.data as User;
};
export const getMe = async (): Promise<User> => {
  const res = await ApiClient.get("users/@me/");
  res.data.createdAt = Date.parse(res.data.createdAt);
  res.data.updatedAt = Date.parse(res.data.updatedAt);
  return res.data as User;
};

export type SkillResponse = {
  id: number;
  name: string;
  createdAt: number; // ms since epoch
};
export const apiGetSkillAll = async (): Promise<SkillResponse[]> => {
  const resp = await ApiClient.get("skills/");
  return resp.data.map((x: any) => {
    return {
      id: x.id,
      userId: x.userId,
      name: x.name,
      createdAt: Date.parse(x.createdAt),
    };
  });
};

export type InterestResponse = {
  id: number;
  name: string;
  createdAt: number; // ms since epoch
};
export const apiGetInterestAll = async (): Promise<InterestResponse[]> => {
    const resp = await ApiClient.get(
        "interests/",
    );
    return resp.data.map((x: any) => {
        return {
            id: x.id,
            name: x.name,
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

////////////////////////////////////////////////
// Definitions for API PUT routes begin here. //
////////////////////////////////////////////////
export const apiPutUserSkillMe = async (skillId: number): Promise<UserSkillResponse> => {
    const resp = await ApiClient.put(
        "users/@me/skills/" + skillId.toString() + "/"
    )
    return {
        id: resp.data.id,
        userId: resp.data.userId,
        skillId: resp.data.skillId,
        createdAt: Date.parse(resp.data.createdAt),
    }
}

export const apiPutUserInterestMe = async (interestId: number): Promise<UserInterestResponse> => {
    const resp = await ApiClient.put(
        "users/@me/interests/" + interestId.toString() + "/"
    )
    return {
        id: resp.data.id,
        userId: resp.data.userId,
        interestId: resp.data.interestId,
        createdAt: Date.parse(resp.data.createdAt),
    }
}


///////////////////////////////////////////////////
// Definitions for API DELETE routes begin here. //
///////////////////////////////////////////////////
export const apiDeleteUserSkillMe = async (skillId: number): Promise<AxiosResponse> => {
    const resp = await ApiClient.delete(
        "users/@me/skills/" + skillId.toString() + "/"
    )
    return resp
}

export const apiDeleteUserInterestMe = async (interestId: number): Promise<AxiosResponse> => {
    const resp = await ApiClient.delete(
        "users/@me/interests/" + interestId.toString() + "/"
    )
    return resp
}

export const apiDeleteProjectMe = async (projectId: number): Promise<AxiosResponse> => {
    const resp = await ApiClient.delete(
        "users/@me/projects/" + projectId.toString() + "/"
    )
    return resp
}