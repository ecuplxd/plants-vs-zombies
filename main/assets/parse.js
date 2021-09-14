function to_cell(strings, prefix) {
  prefix = prefix || 'zombie/';

  let result = {};
  let div = document.createElement('div');

  div.innerHTML = strings;

  let items = Array.from(div.getElementsByTagName('SubTexture'));

  items.forEach((item) => {
    let name = prefix + get_name(item.getAttribute('name'));
    let left = get_size(item.getAttribute('left'));
    let top = get_size(item.getAttribute('top'));
    let width = get_size(item.getAttribute('width'));
    let height = get_size(item.getAttribute('height'));
    let cell = result[name];
    let info = { left, top, width, height };

    if (cell) {
      cell.push(info);
    } else {
      result[name] = [info];
    }
  });

  console.log(JSON.stringify(result));
}

function get_name(str) {
  return str.split('.')[0].split('-')[0].split('_')[0];
}

function get_size(size) {
  return parseFloat(size);
}
