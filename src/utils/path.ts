const getFileName = (path: string) => {
  return path.split('/').pop();
};

const getTruncatedFilePath = (path: string) => {
  const pathArray = path.split('/');
  const fileName = pathArray.pop();
  const fileDir = pathArray.pop();
  const fileParentDir = pathArray.pop();
  return `${fileParentDir}/${fileDir}/${fileName}`;
};

export { getFileName, getTruncatedFilePath };
