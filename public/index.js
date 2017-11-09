$(function() {
  var $form = $('#form');
  var $content = $('#content');
  $form.on('submit', function(e) {
    e.preventDefault();
    $.ajax({
      method: 'GET',
      url: '/readability',
      data: { url: e.target.url.value },
    }).done(function(res) {
      console.log(res);
      $content.html(res);
    });
  });
});
