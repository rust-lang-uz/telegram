export default (input: string) =>
  input.replaceAll(/[\u00A0-\u9999<>\&]/g, (i) => {
    return "&#" + i.charCodeAt(0) + ";";
  });
