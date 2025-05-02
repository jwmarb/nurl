import { BACKEND_URL } from './constants';
import { StatusCodes } from 'http-status-codes';
import axios from 'axios';

type APIResponse<T = unknown> = {
  error: string | null;
  data: T;
};

type RegisterAPIResponse = APIResponse<{ target_field: string } | null>;
type LoginAPIResponse = APIResponse<{ token: string }>;

class API {
  private api = axios.create({
    baseURL: BACKEND_URL,
  });

  public async isAlive(signal?: AbortSignal): Promise<boolean> {
    try {
      const response = await this.api.get(`/health`, { signal });

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

  public async login(username: string, password: string, rememberMe: boolean): Promise<LoginAPIResponse> {
    try {
      const response = await this.api.post('/api/auth', {
        username,
        password,
        remember_me: rememberMe,
      });
      return response.data;
    } catch (e) {
      if (axios.isAxiosError(e)) {
        return e.response?.data as LoginAPIResponse;
      }

      return {
        error: 'An unexpected error occurred. Please try again later.',
      } as LoginAPIResponse;
    }
  }

  public async register(username: string, password: string, confirmPassword: string): Promise<RegisterAPIResponse> {
    try {
      await this.api.post('/api/register', {
        username,
        password,
        confirm_password: confirmPassword,
      });
      return {
        error: null,
        data: null,
      };
    } catch (e) {
      if (axios.isAxiosError(e)) {
        return e.response?.data as RegisterAPIResponse;
      }

      return {
        error: 'An unexpected error occurred. Please try again later.',
      } as RegisterAPIResponse;
    }
  }
}

export default new API();
