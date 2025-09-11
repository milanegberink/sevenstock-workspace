import { z } from 'zod';

export const userForLogin = z.object({
	username: z.string().min(5, 'Username should be 5 charecters minimum.'),
	password: z.string().min(8, 'Password should be 8 charecters minimum.'),
	email: z.email()
});
