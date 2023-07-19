import type { User } from '..';
import { JWT_KEY } from './constant';

interface APIResponse<T> {
  message: T;
  error: string;
}

const baseUrl = () => {
  return '/api';
};

const handleResponse = async <T>(response: Response): Promise<T> => {
  let jsonResponse: APIResponse<T>;
  try {
    jsonResponse = await response.json();
  } catch (e: any) {
    throw new Error(e);
  }
  return jsonResponse.message;
};

const headerBuilder = function () {
  let isJson = false;
  let withAuth = false;
  return {
    withAuth: function () {
      withAuth = true;
      return this;
    },
    json: function () {
      isJson = true;
      return this;
    },
    build: function (): HeadersInit {
      let header: HeadersInit = {};
      if (withAuth) {
        header.Authorization = `Bearer ${localStorage.getItem(JWT_KEY)}`;
      }
      if (isJson) {
        header['Content-Type'] = 'application/json';
      }
      return header;
    },
  };
};

const api = {
  signup: async (user: User): Promise<string> => {
    const response = await fetch(baseUrl() + '/signup', {
      method: 'POST',
      headers: headerBuilder().json().build(),
      body: JSON.stringify(user),
    });
    return handleResponse(response);
  },
  signin: async (user: User): Promise<string> => {
    const response = await fetch(baseUrl() + '/signin', {
      method: 'POST',
      headers: headerBuilder().json().build(),
      body: JSON.stringify(user),
    });
    return handleResponse(response);
  },
  retrieve: async (key: string): Promise<string> => {
    const response = await fetch(baseUrl() + `/retrieve?key=${key}`, {
      method: 'GET',
      headers: headerBuilder().json().withAuth().build(),
    });
    return handleResponse(response);
  },
  store: async (key: string, value: string): Promise<string> => {
    const response = await fetch(baseUrl() + '/store', {
      method: 'POST',
      headers: headerBuilder().json().withAuth().build(),
      body: JSON.stringify({ key, value }),
    });
    return handleResponse(response);
  },
};

export default api;
