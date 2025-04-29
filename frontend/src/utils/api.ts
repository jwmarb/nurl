import { BACKEND_URL } from './constants';
import { StatusCodes } from 'http-status-codes';

class API {
  public async isAlive(signal?: AbortSignal): Promise<boolean> {
    try {
      const response = await fetch(`${BACKEND_URL}/health`, { signal });

      if (signal) {
        return signal.aborted && response.status === StatusCodes.OK;
      }

      return response.status === StatusCodes.OK;
    } catch {
      return signal?.aborted ?? false;
    }
  }

  public async isAuthenticated(signal?: AbortSignal): Promise<boolean> {
    // TODO: Implement isAuthenticated method
    return true;
  }
}

export default new API();
