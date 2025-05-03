import { BACKEND_URL } from './constants';
import { StatusCodes } from 'http-status-codes';
import axios from 'axios';
import { useAuthStore } from '$/store/auth';

type APIResponse<T = unknown> = {
  error: string | null;
  data: T;
};

type RegisterAPIResponse = APIResponse<{ target_field: string } | null>;
type LoginAPIResponse = APIResponse<{ token: string }>;

export type ShortenedURL = {
  id: string;
  original_url: string;
  short_url: string;
  expiry_date?: string;
  created_at: string;
  updated_at: string;
  owner: string;
  redirects: number;
};

export type ShortenURLData = {
  original_url: string;
  custom_path?: string;
  expiration?: number;
};

export type UpdateURLRequest = ShortenURLData & {
  id: string;
};

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

  public async isAuthenticated(token: string | null): Promise<boolean> {
    try {
      const response = await this.api.get('/api/auth', {
        headers: { Authorization: `Bearer ${token}` },
      });
      return response.status === StatusCodes.OK;
    } catch {
      return false;
    }
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
        data: { token: '' },
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
        data: null,
      } as RegisterAPIResponse;
    }
  }

  // New methods for URL shortening API
  public async getShortenedURLs(): Promise<APIResponse<ShortenedURL[]>> {
    try {
      const response = await this.api.get('/api/shorten', {
        headers: { Authorization: `Bearer ${useAuthStore.getState().token}` },
      });
      return response.data;
    } catch (e) {
      if (axios.isAxiosError(e)) {
        return e.response?.data as APIResponse<ShortenedURL[]>;
      }

      return {
        error: 'An unexpected error occurred. Please try again later.',
        data: [],
      } as APIResponse<ShortenedURL[]>;
    }
  }

  public async createShortenedURL(urlData: ShortenURLData): Promise<APIResponse<ShortenedURL>> {
    try {
      const response = await this.api.post('/api/shorten', urlData, {
        headers: { Authorization: `Bearer ${useAuthStore.getState().token}` },
      });
      return response.data;
    } catch (e) {
      if (axios.isAxiosError(e)) {
        return e.response?.data as APIResponse<ShortenedURL>;
      }

      return {
        error: 'An unexpected error occurred. Please try again later.',
        data: {} as ShortenedURL,
      } as APIResponse<ShortenedURL>;
    }
  }

  public async updateShortenedURL(urlData: UpdateURLRequest): Promise<APIResponse<ShortenedURL>> {
    try {
      const response = await this.api.put('/api/shorten', urlData, {
        headers: { Authorization: `Bearer ${useAuthStore.getState().token}` },
      });
      return response.data;
    } catch (e) {
      if (axios.isAxiosError(e)) {
        return e.response?.data as APIResponse<ShortenedURL>;
      }

      return {
        error: 'An unexpected error occurred. Please try again later.',
        data: {} as ShortenedURL,
      } as APIResponse<ShortenedURL>;
    }
  }

  public async deleteShortenedURL(id: string): Promise<APIResponse<null>> {
    try {
      const response = await this.api.delete(`/api/shorten/${id}`, {
        headers: { Authorization: `Bearer ${useAuthStore.getState().token}` },
      });
      return response.data;
    } catch (e) {
      if (axios.isAxiosError(e)) {
        return e.response?.data as APIResponse<null>;
      }

      return {
        error: 'An unexpected error occurred. Please try again later.',
        data: null,
      } as APIResponse<null>;
    }
  }
}

export default new API();
