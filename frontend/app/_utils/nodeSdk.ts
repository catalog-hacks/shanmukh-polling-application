import { Config, SDK } from '@corbado/node-sdk';

let sdkInstance: SDK | null = null;

export default function getNodeSDK() {
    if (!sdkInstance) {
        const projectID = process.env.NEXT_PUBLIC_CORBADO_PROJECT_ID!;
        const apiSecret = process.env.CORBADO_API_SECRET!;
        const frontendApi = process.env.FRONTEND_API!;
        const backendApi = process.env.BACKEND_API!;
        const config = new Config(projectID, apiSecret, frontendApi, backendApi);
        sdkInstance = new SDK(config);
    }
    return sdkInstance;
}
