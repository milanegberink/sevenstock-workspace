import { z } from 'zod';

export const tokenClaims = z.object({
	exp: z.number(),
	iat: z.number(),
	sub: z.uuidv7(),
	ident: z.string().optional(),
	org: z.string().optional(),
	avatar: z.url().optional(),
	email: z.email().optional()
});

export type TokenClaims = z.infer<typeof tokenClaims>;
