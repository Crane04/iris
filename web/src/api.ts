import axios from "axios";

import backendUrl from "./constant";

const post = async (path: string, data: {}): Promise<any> => {
  try {
    const response = await axios.post(`${backendUrl}/${path}`, data);
    return response.data;
  } catch (error) {
    throw error;
  }
};

export default post;
