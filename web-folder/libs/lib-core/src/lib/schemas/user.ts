import { z } from 'zod';

export const loginPayload = z.object({
	email: z.email(),
	pwd: z.string().min(8, 'Password should be 8 charecters minimum.')
});

export const user = z.object({
	email: z.email(),
	ident: z.string(),
	avatar: z.url().optional(),
	org: z.string().optional()
});

export type LoginPayload = z.infer<typeof loginPayload>;

export type LoginResponse = {
	access_token: string;
};

export type User = z.infer<typeof user>;
