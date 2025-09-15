import { z } from 'zod';

export const userForLogin = z.object({
	password: z.string().min(8, 'Password should be 8 charecters minimum.'),
	email: z.email()
});

export const userForCreate = z.object({
	username: z.string().min(5, 'Username should be 5 charecters minimum.'),
	email: z.email(),
	password: z.string().min(8, 'Password should be 8 charecters minimum.')
});

export const user = z.object({
	email: z.email(),
	ident: z.string(),
	avatar: z.url().optional(),
	org: z.string().optional()
});

export type UserForLogin = z.infer<typeof userForLogin>;

export type User = z.infer<typeof user>;
