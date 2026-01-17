import type { LoginPayload } from './user.ts';

export enum SWReqType {
	SetToken = 'SET_TOKEN',
	GetUser = 'GET_USER',
	LoginRequest = 'LOGIN_REQUEST'
}

export type SWRequest = SetTokenRequest | LoginUserRequest;

interface SetTokenRequest {
	type: SWReqType.SetToken;
}

interface LoginUserRequest {
	type: SWReqType.LoginRequest;
	payload: LoginPayload;
}

export const setTokenRequest: SetTokenRequest = {
	type: SWReqType.SetToken
};

export const loginUserRequest = (payload: LoginPayload): LoginUserRequest => ({
	type: SWReqType.LoginRequest,
	payload
});
