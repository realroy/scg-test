const ENDPOINT = "/api/xzy";
const KEY = "xyz";
const EXPIRED_IN = 1000 * 60 * 60 * 24 * 7

export const doSCGService = async () => {
  const cache = localStorage.getItem(KEY);
  let data = JSON.parse(cache)
  
  if (data && data.result && Date.now() > data.expiredAt) {
    return data.result;
  }

  const res = await fetch(ENDPOINT);
  const result = res.json();

  const expiredAt = Date.now() + EXPIRED_IN
  
  data = { expiredAt, result }
  
  localStorage.setItem(KEY, JSON.stringify(data));

  return result;
};

export default doSCGService;
