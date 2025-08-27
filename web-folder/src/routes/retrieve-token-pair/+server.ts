import { json, type RequestHandler } from '@sveltejs/kit';
import grpc from '@grpc/grpc-js';
import protoLoader from '@grpc/proto-loader';

const PROTO_PATH = '../proto/refresh_tokens.proto';

const packageDef = protoLoader.loadSync(PROTO_PATH, {});
const proto = grpc.loadPackageDefinition(packageDef);

const client = new proto.auth.v1.Auth('localhost:50051', grpc.credentials.createInsecure());

export const GET: RequestHandler = ({ cookies }) => {
	const req = { refreshToken: 'your_access_token_here' };

	const refresh = cookies.get('refresh');

	client.RefreshToken(req, (err, response) => {
		console.log(response);
	});

	return json({ token: refresh });
};
