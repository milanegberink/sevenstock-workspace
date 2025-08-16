import { jwtVerify, importSPKI } from 'jose';
import fs from 'node:fs/promises';
import path from 'node:path';
import type { Handle } from '@sveltejs/kit';

const publicKey = await getKey();

export const handle: Handle = async ({ event, resolve }) => {
	try {
		const { payload } = await jwtVerify(
			'eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJzdWIiOiIwMTk4MDBkYS0xZDRmLTcxYzItODBmNy04N2MxNTQ3Y2VhY2IiLCJleHAiOjE3NTIzNjE4NTEsImlhdCI6MTc1MjM2MDk1MX0.LrZwHN-tpCgZOhJY4qUmbNIa_PTt6Tc-S5omRLSQMlMw80l1sbCS5unZFLplnweZmuvkrlu9ZTciY2vaSx81CQ',
			publicKey
		);
		console.log('Success');
	} catch (err) {
		console.log('Failed', err);
	}

	const response = await resolve(event);
	return response;
};

async function getKey() {
	console.log('Retrieved public key');
	const filePath = path.join(process.cwd(), '..', 'keys', 'access-secrets', 'public-key.pem');
	const spki = await fs.readFile(filePath, 'utf-8');
	const publicKey = await importSPKI(spki, 'EdDSA');
	return publicKey;
}
