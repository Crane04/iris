import axios from "axios";

const backendUrl = import.meta.env.VITE_BACKEND_URL;
console.log(`${backendUrl}`);

const post = async (path: string, data: {}): Promise<any> => {
  try {
    const response = await axios.post(`${backendUrl}/${path}`, data);
    return response.data;
  } catch (error) {
    throw error;
  }
};

export default post;
