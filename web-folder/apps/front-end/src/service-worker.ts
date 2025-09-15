/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />

import { mountServiceWorker } from '@lib/core/service-worker';

const sw = self as unknown as ServiceWorkerGlobalScope;

mountServiceWorker(sw);
