const DEFAULT_EXPIRED_IN = 1000 * 60 * 60 * 24 * 7

export const fetchDataService = async ({
  path,
  key,
  expireIn = DEFAULT_EXPIRED_IN,
  method = 'GET',
  body,
  headers = {
    'Content-Type': 'application/json'
  }
} = {}) => {
  const cache = localStorage.getItem(key);
  
  let data = JSON.parse(cache)
  
  if (data && data.result && Date.now() > data.expiredAt) {
    return data.result;
  }

  if (typeof body !== 'string') {
    body = JSON.stringify(body)
  }

  const API_ENDPOINT = process.env.NODE_ENV !== 'development'
    ? 'api'
    : 'http://localhost:5000'

  const url = `${API_ENDPOINT}/${path}`
  const res = await fetch(url, { method, body, headers });
  const result = res.json();

  const expiredAt = Date.now() + expireIn
  
  data = { expiredAt, result }
  
  localStorage.setItem(key, JSON.stringify(data));

  return result;
}