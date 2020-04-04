const DEFAULT_EXPIRED_IN = 1000 * 60 * 60 * 24 * 7

export const fetchDataService = async (endpoint,key, expireIn = DEFAULT_EXPIRED_IN) => {
  const cache = localStorage.getItem(key);
  
  let data = JSON.parse(cache)
  
  if (data && data.result && Date.now() > data.expiredAt) {
    return data.result;
  }

  const res = await fetch(endpoint);
  const result = res.json();

  const expiredAt = Date.now() + expireIn
  
  data = { expiredAt, result }
  
  localStorage.setItem(key, JSON.stringify(data));

  return result;
}