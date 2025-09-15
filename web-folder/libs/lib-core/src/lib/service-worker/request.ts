import type { UserForLogin } from '$lib/schemas/user.js';

export enum SWReqType {
	SetToken = 'SET_TOKEN',
	GetUser = 'GET_USER',
	LoginRequest = 'LOGIN_REQUEST'
}

export type SWRequest = GetUserRequest | SetTokenRequest | LoginUserRequest;

interface GetUserRequest {
	type: SWReqType.GetUser;
}

interface SetTokenRequest {
	type: SWReqType.SetToken;
}

interface LoginUserRequest {
	type: SWReqType.LoginRequest;
	payload: UserForLogin;
}

export const getUserRequest: GetUserRequest = {
	type: SWReqType.GetUser
};

export const setTokenRequest: SetTokenRequest = {
	type: SWReqType.SetToken
};

export const loginUserRequest = (payload: UserForLogin): LoginUserRequest => ({
	type: SWReqType.LoginRequest,
	payload
});
