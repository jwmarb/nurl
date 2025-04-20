import { BACKEND_URL } from './constants';
import { StatusCodes } from 'http-status-codes';

class API {
  public async isAlive(signal?: AbortSignal): Promise<boolean> {
    const response = await fetch(`${BACKEND_URL}/health`, { signal });

    if (signal) {
      return signal.aborted && response.status === StatusCodes.OK;
    }

    return response.status === StatusCodes.OK;
  }
}

export default new API();
