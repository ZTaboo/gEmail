use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailConfig {
    pub smtp_service: String,
    pub username: String,
    pub password: String,
}

pub struct TemplateData {
    pub default: String,
    pub tmp: String,
}

pub fn init_temp() -> TemplateData {
    let data = TemplateData {
        default: tp_default(),
        tmp: tmp_tm(),
    };
    return data;
}

fn tp_default() -> String {
    let x = r##"<!DOCTYPE html>
<html>

<head>
    <title>注册确认邮件</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>

<body>
<div style="background-color: #f2f2f2; padding: 30px;">
    <div style="background-color: #333; padding: 10px; text-align: center; color: #fff;">
        <h1>欢迎加入我们！</h1>
        <p>感谢您注册 LearnHacker。</p>
    </div>
    <div style="background-color: #fff; padding: 20px; border-radius: 5px; margin-top: 20px;">
        <h2 style="color: #333;">您的注册验证码</h2>
        <p>您的注册验证码是：<strong>123456</strong></p>
        <p>请在注册页面输入此验证码以完成注册。</p>
    </div>
    <div style="display: flex;justify-content: end;padding-right: 20px;">
        <div>
            <p>祝您使用愉快！</p>
            <p>LearnHacker 团队</p>
            <p style="margin-top: 20px;">如果您未注册，请忽略此邮件。</p>
        </div>
    </div>
</div>
</body>
</html>"##;
    x.to_string()
}

fn tmp_tm() -> String {
    let x = r##"<div style="background-color:#1c1d22;">

    <div style="background:#434857;background-color:#434857;margin:0px auto;max-width:600px;">
      <table align="center" border="0" cellpadding="0" cellspacing="0" role="presentation" style="background:#434857;background-color:#434857;width:100%;">
        <tbody>
          <tr>
            <td style="direction:ltr;font-size:0px;padding:0;text-align:center;">

              <div class="mj-column-per-100 mj-outlook-group-fix" style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                <table border="0" cellpadding="0" cellspacing="0" role="presentation" style="vertical-align:top;" width="100%">
                  <tbody>
                    <tr>
                      <td align="center" style="font-size:0px;padding:0;word-break:break-word;">
                        <table border="0" cellpadding="0" cellspacing="0" role="presentation" style="border-collapse:collapse;border-spacing:0px;">
                          <tbody>
                            <tr>
                              <td style="width:600px;">
                                <img height="auto" src="https://res.cloudinary.com/css-tricks/image/upload/c_scale,f_auto,q_auto,w_1100/v1638545296/codepen-spark-header-2022_oih61t.png" style="border:0;display:block;outline:none;text-decoration:none;height:auto;width:100%;font-size:13px;" width="600">
                              </td>
                            </tr>
                          </tbody>
                        </table>
                      </td>
                    </tr>
                    <tr>
                      <td align="center" style="font-size:0px;padding:0;word-break:break-word;">
                        <div style="font-family:Lato, system-ui, sans-serif;font-size:10px;line-height:1;text-align:center;color:#e3e4e8;"><a href="http://post.spmailtechnolo.com/f/a/G9W3wJw9u7Ka4RsB9TLzFQ%7E%7E/AABEfgA%7E/RgRmz13gP0QcaHR0cHM6Ly9jb2RlcGVuLmlvL3NwYXJrLzM1NFcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" class="view-on-web-link" rel="noopener" target="_blank">View this issue on CodePen</a></div>
                      </td>
                    </tr>
                    <tr>
                      <td align="left" style="font-size:0px;padding:10px 25px;padding-top:30px;padding-right:35px;padding-bottom:0;padding-left:35px;word-break:break-word;">
                        <div style="font-family:'Telefon Black', system-ui, sans-serif;font-size:13px;text-align:left;color:#e7f0ff;">
                          <h1>Stretchy Type, CSS Timers, and Poisson Disk Sampling</h1>
                        </div>
                      </td>
                    </tr>
                    <tr>
                      <td align="left" style="font-size:0px;padding:10px 25px;padding-top:0;padding-right:35px;padding-left:35px;word-break:break-word;">
                        <div style="font-family:Lato, system-ui, sans-serif;font-size:13px;line-height:1;text-align:left;color:white;">
                          <div class="intro-description">
                            <p>This week's CodePen community highlights include a stretchy type specimen from jomacz, a tutorial on toggling CSS classes to set timers from Stanko Tadić, and a demonstration of how to use poisson disk sampling to form an image from pimskie. </p>
<p>Plus, Jhey Tompkins gets under the hood of X's "repost" button, and Lisa Catalano shows us how to create a full screen video with text overlay.</p>
                          </div>
                        </div>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>

            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div style="background:#333845;background-color:#333845;margin:0px auto;max-width:600px;">
      <table align="center" border="0" cellpadding="0" cellspacing="0" role="presentation" style="background:#333845;background-color:#333845;width:100%;">
        <tbody>
          <tr>
            <td style="direction:ltr;font-size:0px;padding:20px 0;text-align:center;">

              <div class="mj-column-per-50 mj-outlook-group-fix" style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                <table border="0" cellpadding="0" cellspacing="0" role="presentation" width="100%">
                  <tbody>
                    <tr>
                      <td style="vertical-align:top;padding:15px;">
                        <table border="0" cellpadding="0" cellspacing="0" role="presentation" style="" width="100%">
                          <tbody>
                            <tr>
                              <td align="left" style="font-size:0px;padding:10px 25px;word-break:break-word;">
                                <div style="font-family:Lato, system-ui, sans-serif;font-size:13px;line-height:1;text-align:left;color:#e3e4e8;">
                                  <mj-raw>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/tzYE7KrYtNbckTJbTIfPLg%7E%7E/AABEfgA%7E/RgRmz13gP0QxaHR0cHM6Ly9jb2RlcGVuLmlvL25pY29sYXNqZXNlbmJlcmdlci9wZW4veHhtYnZ4TFcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/Ui3HnE2SWaRjo9eBDQTl_gooey-toggle-switch.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/tzYE7KrYtNbckTJbTIfPLg%7E%7E/AABEfgA%7E/RgRmz13gP0QxaHR0cHM6Ly9jb2RlcGVuLmlvL25pY29sYXNqZXNlbmJlcmdlci9wZW4veHhtYnZ4TFcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">Gooey Toggle Switch</a></div>
                                        <div class="spark-desc">Nicolas Jesenberger brings us a set of gooey toggle switches with a fun, stretchy transition between on and off states. Inspired by a <a href="http://post.spmailtechnolo.com/f/a/y3A9nra3RBBgxUwRr2OHew%7E%7E/AABEfgA%7E/RgRmz13gP0Q8aHR0cHM6Ly9kcmliYmJsZS5jb20vc2hvdHMvMzY0NzAyNy1EYWlseVVJLTAxNS1Pbi1PZmYtU3dpdGNoVwNzcGNCCmTn9NjsZM58kJtSEDE2Njk5Nzk4MkBxcS5jb21YBAAAAAA%7E" rel="noopener" target="_blank">Dribbble shot by domaso</a>.</div>
                                      </div>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/8NkFXq-FJpl7UmFGM8A7Gg%7E%7E/AABEfgA%7E/RgRmz13gP0QoaHR0cHM6Ly9jb2RlcGVuLmlvL2xvZmljb2Rlcy9wZW4vTFlNWWRkZVcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/VtzuJJZZQLetxn1u7Qzc_nailing-arcs.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/8NkFXq-FJpl7UmFGM8A7Gg%7E%7E/AABEfgA%7E/RgRmz13gP0QoaHR0cHM6Ly9jb2RlcGVuLmlvL2xvZmljb2Rlcy9wZW4vTFlNWWRkZVcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">Nailing Arcs</a></div>
                                        <div class="spark-desc">LoFi shares a canvas animation of kinetic arcs in motion, reminiscent of the grooves on vinyl records.</div>
                                      </div>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/HH3MArpkruxqCyxFFidv_w%7E%7E/AABEfgA%7E/RgRmz13gP0Q-aHR0cHM6Ly9tdWZmaW5tYW4uaW8vYmxvZy9jc3MtYW5pbWF0aW9ucy1pbnN0ZWFkLW9mLWpzLXRpbWVycy9XA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/viy8txfRfiSHMfzhGgk1_css-anim-js-timer.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/HH3MArpkruxqCyxFFidv_w%7E%7E/AABEfgA%7E/RgRmz13gP0Q-aHR0cHM6Ly9tdWZmaW5tYW4uaW8vYmxvZy9jc3MtYW5pbWF0aW9ucy1pbnN0ZWFkLW9mLWpzLXRpbWVycy9XA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Using CSS animations instead of JavaScript timers</a></div>
                                        <div class="spark-desc">"What if I told you that we can make a timer without using setTimeout, setInterval or requestAnimationFrame? JavaScript is still necessary, but we can create the timer just by toggling some CSS classes." Stanko Tadić shows you how in a tutorial filled with handy embedded Pens.</div>
                                      </div>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/VFzYDBkeranOlPMTgVnIlA%7E%7E/AABEfgA%7E/RgRmz13gP0QjaHR0cHM6Ly9jb2RlcGVuLmlvL2poM3kvcGVuL0tLYnBlQlFXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/ojugFwbPQCilwIgJIkHb_repost-button.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/VFzYDBkeranOlPMTgVnIlA%7E%7E/AABEfgA%7E/RgRmz13gP0QjaHR0cHM6Ly9jb2RlcGVuLmlvL2poM3kvcGVuL0tLYnBlQlFXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Repost Button</a></div>
                                        <div class="spark-desc">Jhey Tompkins recreates the X "repost" button with with a CSS icon sprite and filtering. Try playing with the checkbox controls to get a deeper look at how it works.</div>
                                      </div>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/If97Af4p9M6kZ8sI-OypXw%7E%7E/AABEfgA%7E/RgRmz13gP0QmaHR0cHM6Ly9jb2RlcGVuLmlvL3BpbXNraWUvcGVuL1l6ZFhRWlpXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/I9ytZDcrSySQxPhDtbN2_poisson-disk-sampling.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/If97Af4p9M6kZ8sI-OypXw%7E%7E/AABEfgA%7E/RgRmz13gP0QmaHR0cHM6Ly9jb2RlcGVuLmlvL3BpbXNraWUvcGVuL1l6ZFhRWlpXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Poisson Disk Sampling</a></div>
                                        <div class="spark-desc">pimskie uses a JS implementation of fast poisson disk sampling in arbitrary dimensions to paint a portrait in pointillist style. Check out the Pen description for a link to more detail on this algorithm, and a look at the original photo.</div>
                                      </div>

                                      <div class="spark-item" data-type="collection">
                                        <a href="http://post.spmailtechnolo.com/f/a/ruWx190G3w5mPoAqap_r4Q%7E%7E/AABEfgA%7E/RgRmz13gP0QkaHR0cHM6Ly9jb2RlcGVuLmlvL2NvbGxlY3Rpb24vdkJNUndrVwNzcGNCCmTn9NjsZM58kJtSEDE2Njk5Nzk4MkBxcS5jb21YBAAAAAA%7E" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/1nWDxhE3RzW5KwBNIwrC_cpc-photo-zoom.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">collection</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/ruWx190G3w5mPoAqap_r4Q%7E%7E/AABEfgA%7E/RgRmz13gP0QkaHR0cHM6Ly9jb2RlcGVuLmlvL2NvbGxlY3Rpb24vdkJNUndrVwNzcGNCCmTn9NjsZM58kJtSEDE2Njk5Nzk4MkBxcS5jb21YBAAAAAA%7E" rel="noopener" target="_blank">#CodePenChallenge: Photo Zoom</a></div>
                                        <div class="spark-desc">We zoomed through the third week of the Photos challenge! Flip through our collection from week three, including Pens by <a href="http://post.spmailtechnolo.com/f/a/RXNMrA_WQwE_grUY-OUTQw%7E%7E/AABEfgA%7E/RgRmz13gP0QoaHR0cHM6Ly9jb2RlcGVuLmlvL2lzbWFpbHZ0bC9wZW4vZ09acGVXWFcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">Ismail Vittal</a>, <a href="http://post.spmailtechnolo.com/f/a/OA0gX62ORGt4EKCCOiHnCA%7E%7E/AABEfgA%7E/RgRmz13gP0QmaHR0cHM6Ly9jb2RlcGVuLmlvL1RhbHVza2EvcGVuL1BvWHdYUExXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Taluska</a>, <a href="http://post.spmailtechnolo.com/f/a/wW-lXahFVG6XOy_GVzY56A%7E%7E/AABEfgA%7E/RgRmz13gP0QqaHR0cHM6Ly9jb2RlcGVuLmlvL0p1eHRvcHBvc2VkL3Blbi9Wd1ZvRUJyVwNzcGNCCmTn9NjsZM58kJtSEDE2Njk5Nzk4MkBxcS5jb21YBAAAAAA%7E" rel="noopener" target="_blank">Juxtopposed</a>, and <a href="http://post.spmailtechnolo.com/f/a/X1dOPeT-POAfW5JO5qROFg%7E%7E/AABEfgA%7E/RgRmz13gP0QoaHR0cHM6Ly9jb2RlcGVuLmlvL0RleUpvcmRhbi9wZW4vTVdad1JNbVcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">DeyJordan</a>.</div>
                                      </div>

                                  </mj-raw>
                                </div>
                              </td>
                            </tr>
                          </tbody>
                        </table>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>

              <div class="mj-column-per-50 mj-outlook-group-fix" style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                <table border="0" cellpadding="0" cellspacing="0" role="presentation" width="100%">
                  <tbody>
                    <tr>
                      <td style="vertical-align:top;padding:15px 20px 0 5px;">
                        <table border="0" cellpadding="0" cellspacing="0" role="presentation" style="" width="100%">
                          <tbody>
                            <tr>
                              <td align="left" style="font-size:0px;padding:10px 25px;word-break:break-word;">
                                <div style="font-family:Lato, system-ui, sans-serif;font-size:13px;text-align:left;color:#e3e4e8;">
                                  <mj-raw>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/fErVUq-a5MT3KEMvR2bbDg%7E%7E/AABEfgA%7E/RgRmz13gP0QmaHR0cHM6Ly9jb2RlcGVuLmlvL21pb2NlbmUvcGVuL1dOTFFLRUpXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/46h0j74RReieckPEyCLv_css-fancy-loader.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/fErVUq-a5MT3KEMvR2bbDg%7E%7E/AABEfgA%7E/RgRmz13gP0QmaHR0cHM6Ly9jb2RlcGVuLmlvL21pb2NlbmUvcGVuL1dOTFFLRUpXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Sunday CSS #9: Pure CSS Fancy Loader</a></div>
                                        <div class="spark-desc">Julia Miocene continues her wonderful
<a href="http://post.spmailtechnolo.com/f/a/IKSyVUw3SSI6z7tiDbM0Ww%7E%7E/AABEfgA%7E/RgRmz13gP0QcaHR0cHM6Ly95b3V0dS5iZS9PUnhKLWx1ODllOFcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">CSS tutorial series on YouTube</a> with this animated rainbow dot loader.</div>
                                      </div>

                                      <div class="spark-item" data-type="sponsor">
                                        <a href="http://post.spmailtechnolo.com/f/a/5bbcFIqPKcEB4A5I7KAJww%7E%7E/AABEfgA%7E/RgRmz13gP0SCaHR0cHM6Ly9zcnYuYnV5c2VsbGFkcy5jb20vYWRzL2xvbmcveC9USElJSFVJNlRUVFRUVDRSWlhLTlZUVFRUVFQzR1Y1VUtFVFRUVFRUSzRLMkE3VFRUVFRUVEJMQkNFQTZBN1o2NUpCQ0tXWkRLSExVNU5QM0s3UUNQU1pWUU1WVFcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/Kh9miKC5RKOY0UCwCP3C_slack-logo-horchata-600x600.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">sponsor</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/5bbcFIqPKcEB4A5I7KAJww%7E%7E/AABEfgA%7E/RgRmz13gP0SCaHR0cHM6Ly9zcnYuYnV5c2VsbGFkcy5jb20vYWRzL2xvbmcveC9USElJSFVJNlRUVFRUVDRSWlhLTlZUVFRUVFQzR1Y1VUtFVFRUVFRUSzRLMkE3VFRUVFRUVEJMQkNFQTZBN1o2NUpCQ0tXWkRLSExVNU5QM0s3UUNQU1pWUU1WVFcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">Get 50% off Slack Pro</a></div>
                                        <div class="spark-desc">Take your business to the next level. Slack helps you stay organized, save time &amp; get more done.</div>
                                      </div>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/ZBUlyTT0NHDBxGfc8bDySQ%7E%7E/AABEfgA%7E/RgRmz13gP0QlaHR0cHM6Ly9jb2RlcGVuLmlvL2pvbWFjei9wZW4vTVdaS0tFWVcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/QeDixchKQQhkiUoYrPga_stretched.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/ZBUlyTT0NHDBxGfc8bDySQ%7E%7E/AABEfgA%7E/RgRmz13gP0QlaHR0cHM6Ly9jb2RlcGVuLmlvL2pvbWFjei9wZW4vTVdaS0tFWVcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">Stretched</a></div>
                                        <div class="spark-desc">jomacz brings a bit of elastic excitement to a text specimen with GreenSock's MorphSVG plugin in this fun typographic experiment.</div>
                                      </div>

                                      <div class="spark-item" data-type="sponsor">
                                        <a href="http://post.spmailtechnolo.com/f/a/ElN4KwuHoLwet24lZ9d5mA%7E%7E/AABEfgA%7E/RgRmz13gP0SCaHR0cHM6Ly9zcnYuYnV5c2VsbGFkcy5jb20vYWRzL2xvbmcveC9UQ0RSUTdTNlRUVFRUVERBVURFQ1ZUVFRUVFQ0TkpGNUtFVFRUVFRURTNYNTU3VFRUVFRUVEo0TFA3NkNMV0RWMkhNNktCSVpWSERVUEJEMktRREQ2QjRMS0pJRVcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/rsCNGzwzT4C1uyO17jYJ_understanding-streams-in-redis-and-kafka-600x600.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">sponsor</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/ElN4KwuHoLwet24lZ9d5mA%7E%7E/AABEfgA%7E/RgRmz13gP0SCaHR0cHM6Ly9zcnYuYnV5c2VsbGFkcy5jb20vYWRzL2xvbmcveC9UQ0RSUTdTNlRUVFRUVERBVURFQ1ZUVFRUVFQ0TkpGNUtFVFRUVFRURTNYNTU3VFRUVFRUVEo0TFA3NkNMV0RWMkhNNktCSVpWSERVUEJEMktRREQ2QjRMS0pJRVcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">Understanding Streams in Redis and Kafka</a></div>
                                        <div class="spark-desc">Stream processing can still be a very complex topic. To make it really simple, we have gone through the depths of two streaming systems, Kafka and Redis Streams, and have created over 50 illustrations to make it easy to understand.</div>
                                      </div>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/dVXER6XP1zsUt0BandukHA%7E%7E/AABEfgA%7E/RgRmz13gP0QsaHR0cHM6Ly9jb2RlcGVuLmlvL2dhbWJoaXJzaGFybWEvcGVuL1J3RVBqUEtXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/T7bG4AiOQsazZ0hZDu8G_moon-chandrayaan.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/dVXER6XP1zsUt0BandukHA%7E%7E/AABEfgA%7E/RgRmz13gP0QsaHR0cHM6Ly9jb2RlcGVuLmlvL2dhbWJoaXJzaGFybWEvcGVuL1J3RVBqUEtXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Moon | CSS | ~ Chandrayaan-3</a></div>
                                        <div class="spark-desc">Gambhir Sharma celebrates the "day when Chandrayan 3 will land on Moon &amp; also the match [between] Pragyan &amp; Magnus" with this lovely CSS animation of the moon.</div>
                                      </div>

                                      <div class="spark-item" data-type="pen">
                                        <a href="http://post.spmailtechnolo.com/f/a/94dh-JC6ByEKBOeI5vIdnA%7E%7E/AABEfgA%7E/RgRmz13gP0QlaHR0cHM6Ly9jb2RlcGVuLmlvL2xpc2FfYy9wZW4vTFlNUGV4dlcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">
                                          <img class="spark-thumb" src="https://production-codepen-email-assets.s3-us-west-2.amazonaws.com/production/FQwfELzXQASJeGE1nF0I_full-screen-video-text.png" height="" width="260">
                                        </a>
                                        <div class="spark-item-type">pen</div>
                                        <div class="spark-title"><a href="http://post.spmailtechnolo.com/f/a/94dh-JC6ByEKBOeI5vIdnA%7E%7E/AABEfgA%7E/RgRmz13gP0QlaHR0cHM6Ly9jb2RlcGVuLmlvL2xpc2FfYy9wZW4vTFlNUGV4dlcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">Full Screen Video with Text Overlay</a></div>
                                        <div class="spark-desc">Lisa Catalano demonstrates how to make
a full screen video as a background at any screen size, with a combo of "absolute positioning, full viewport width and height, and 'object-fit: cover;'"</div>
                                      </div>

                                  </mj-raw>
                                </div>
                              </td>
                            </tr>
                          </tbody>
                        </table>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>

            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div style="background:#131417;background-color:#131417;margin:0px auto;max-width:600px;">
      <table align="center" border="0" cellpadding="0" cellspacing="0" role="presentation" style="background:#131417;background-color:#131417;width:100%;">
        <tbody>
          <tr>
            <td style="direction:ltr;font-size:0px;padding:20px 0;padding-top:15px;text-align:center;">

              <div class="mj-column-per-100 mj-outlook-group-fix" style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                <table border="0" cellpadding="0" cellspacing="0" role="presentation" width="100%">
                  <tbody>
                    <tr>
                      <td style="vertical-align:top;padding:15px;">
                        <table border="0" cellpadding="0" cellspacing="0" role="presentation" style="" width="100%">
                          <tbody>
                            <tr>
                              <td align="left" style="font-size:0px;padding:10px 25px;word-break:break-word;">
                                <div style="font-family:Lato, system-ui, sans-serif;font-size:13px;text-align:left;color:#e3e4e8;">
                                  <mj-raw>
                                    <h2 class="news-header" style="margin: 0 0 5px 0;">
                                      <img src="https://assets.codepen.io/t-1/chriscorner.png" style="width: 65px;"> Chris’ Corner
                                    </h2>
                                    <p style="color: #9dabc9; margin: 0 0 15px 0;font-size: 16px;">A collection of web design and development news and thoughts from CodePen's own Chris Coyier.</p>
                                    <div class="news-bar"></div>
                                  </mj-raw>
                                  <mj-raw>
                                    <div class="news-content">
                                      <p>SVG has so many tricks up its sleeve. It’s really a full-featured drawing API literally designed for the web, but few of us really truly understand it nor reach for it enough. Heck, I even <a href="http://post.spmailtechnolo.com/f/a/bVbx5ISvMG_yopggyeFo-g%7E%7E/AABEfgA%7E/RgRmz13gP0QtaHR0cHM6Ly9hYm9va2FwYXJ0LmNvbS9wcm9kdWN0cy9wcmFjdGljYWwtc3ZnVwNzcGNCCmTn9NjsZM58kJtSEDE2Njk5Nzk4MkBxcS5jb21YBAAAAAA%7E" rel="noopener" target="_blank">wrote a book about it</a>, and I don’t. At the time, just getting people to use SVG for icons felt like an uphill battle, but thankfully, I think that one has been won.</p>
<p>Let’s look at some cool SVG examples that have crossed my desk lately.</p>
<hr>
<h3><a href="http://post.spmailtechnolo.com/f/a/lUDYKcjnZb2gPblIeiMT3Q%7E%7E/AABEfgA%7E/RgRmz13gP0Q-aHR0cHM6Ly9jaHJpc2tpcmtuaWVsc2VuLmNvbS9ibG9nL2FuaW1hdGUtYW4tc3ZnLWlubmVyLXN0cm9rZS9XA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Animate an SVG Shape’s Inner Stroke</a></h3>
<p>Christopher Kirk-Nielsen with a variety of demos that are a good reminder that <code>stroke</code> is animatable (like the width of it), which can do some cool effects. <a href="http://post.spmailtechnolo.com/f/a/lUDYKcjnZb2gPblIeiMT3Q%7E%7E/AABEfgA%7E/RgRmz13gP0Q-aHR0cHM6Ly9jaHJpc2tpcmtuaWVsc2VuLmNvbS9ibG9nL2FuaW1hdGUtYW4tc3ZnLWlubmVyLXN0cm9rZS9XA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">This demo</a> looks like the classic one where the stroke moves to the inside only, filling the shapes.</p>
<img src="https://res.cloudinary.com/css-tricks/image/upload/c_scale,w_375,h_375,dpr_2/f_auto,q_auto/v1693000168/CleanShot-2023-08-25-at-14.48.58.gif?_i=AA" width="550">
<p>It was only in the last few years that browsers like Chrome <a href="http://post.spmailtechnolo.com/f/a/NsP_yjbtr2cuF1xv_lErww%7E%7E/AABEfgA%7E/RgRmz13gP0RCaHR0cHM6Ly9kZXZlbG9wZXIuY2hyb21lLmNvbS9ibG9nL2hhcmR3YXJlLWFjY2VsZXJhdGVkLWFuaW1hdGlvbnMvVwNzcGNCCmTn9NjsZM58kJtSEDE2Njk5Nzk4MkBxcS5jb21YBAAAAAA%7E" rel="noopener" target="_blank">GPU accelerated SVG animations</a>, making stuff like this really smooth.</p>
<h3><a href="http://post.spmailtechnolo.com/f/a/tORbVUmtGy9GbFnt128OAA%7E%7E/AABEfgA%7E/RgRmz13gP0QpaHR0cHM6Ly95dWFuY2h1YW4uZGV2L3N2Zy12aWV3Ym94LXBhZGRpbmdXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">SVG viewBox padding</a></h3>
<p>To be clear, the <code>viewBox</code> in SVG does not actually have padding. But it’s an important thing to think about. Chuan makes the point that if you make a 10✕10 area via the <code>viewBox</code>, then make a <code>&lt;rect&gt;</code> that fills that 10✕10 area, the <code>stroke</code> around it, the stroke will be half cut off. That’s because stroke straddles the edge of shapes in SVG. So you either gotta monkey with the coordinates of the shapes, or you gotta adjust the <code>viewBox</code> to handle it. Chuan’s thinking is: let a processor handle it.</p>
<pre><code>viewBox="0 0 10 10 padding .5"

/_ translates to _/

viewBox="-.5 -.5 11 11"
</code></pre>
<p>Clever thinking, really. The <a href="http://post.spmailtechnolo.com/f/a/SVkraHY_zawtb9t3rJtzyw%7E%7E/AABEfgA%7E/RgRmz13gP4SuAWh0dHBzOi8vY3NzLWRvb2RsZS5jb20vc3ZnLz9jb2RlPXN2ZyslN0IlMEErK3ZpZXdCb3glM0ErMCswKzEwKzEwK3BhZGRpbmcrLjIlM0IlMEErKyUwQSsrc3R5bGUrYm9yZGVyJTNBKzFweCtkYXNoZWQlM0IlMEErK3N0cm9rZSUzQSslMjMwMDAlM0IlMEErK3N0cm9rZS13aWR0aCUzQSsuNCUzQiUwQSsrc3Ryb2tlLWxpbmVjYXAlM0Ercm91bmQlM0IlMEErKyUwQSsrbGluZSoxMHgxMCslN0IlMEErKysreDElMkMreTElMkMreDIlMkMreTIlM0ErJTQwcCUyOCUwQSsrKysrKyU0MG54JTI4LTElMjkrJTQwbnklMjgtMSUyOSslNDBueCslNDBueSUyQyUwQSsrKysrKyU0MG54KyU0MG55JTI4LTElMjkrJTQwbnglMjgtMSUyOSslNDBueSUyQyUwQSsrKysrKyU0MG54KyU0MG55JTI4LTElMjkrJTQwbngrJTQwbnklMEErKysrJTI5JTNCJTBBKyslN0QlMEElN0RXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">CSS Doodle tool can do it</a>.</p>
<hr>
<h3><a href="http://post.spmailtechnolo.com/f/a/ZCn4uSv2TZhPTAAfhO1pkg%7E%7E/AABEfgA%7E/RgRmz13gP0RJaHR0cHM6Ly9jbG91ZGZvdXIuY29tL3RoaW5rcy9zby15b3UtY2FuLXNldC1hbi1zdmctY2lyY2xlcy1yYWRpdXMtaW4tY3NzL1cDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">So… you can set an SVG circle’s radius in CSS?</a></h3>
<p>The very basic answer to Paul Hebert’s questions is: yeah, totally. Like if you have this:</p>
<pre><code>&lt;svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg"&gt;
  &lt;circle cx="50" cy="50" r="50" /&gt;
&lt;/svg&gt;
</code></pre>
<p>You can adjust the radius in CSS like:</p>
<pre><code>circle {
  r: 20;
}
</code></pre>
<p>Not CSS you see every day, but yeah, that’s totally fine. But Paul’s point is that <em>normally</em> you set the radius in the SVG code, but you might need it in CSS code. Like, a way to keep them in sync is good. In <a href="http://post.spmailtechnolo.com/f/a/EgWxtRLFXVKyioVMnrCGxA%7E%7E/AABEfgA%7E/RgRmz13gP0QmaHR0cHM6Ly9jb2RlcGVuLmlvL3BoZWJlcnQvcGVuL1BvQkVKVnBXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Paul’s demo</a>, it looks like he doesn’t even set the radius in SVG at all, just does it in CSS via a <code>--radius</code> Custom Property, then uses that in the other calculations needed to make these percentage meters work.</p>
<img src="https://res.cloudinary.com/css-tricks/image/upload/c_scale,w_354,h_260,dpr_2/f_auto,q_auto/v1693005611/Screenshot-2023-08-25-at-4.19.13-PM.png?_i=AA" width="550">
<h3><a href="http://post.spmailtechnolo.com/f/a/OkJAGVpiNUMXyrnU7nLwvQ%7E%7E/AABEfgA%7E/RgRmz13gP0QdaHR0cHM6Ly93d3cubmFuLmZ5aS9zdmctcGF0aHNXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Understanding SVG Paths</a></h3>
<p>The <code>&lt;path&gt;</code> element in SVG is the most complicated of the shape drawing elements. In fact, as I understand it, all the other elements are just syntactic sugar over a path anyway. I once wrote An Illustrated Guide when I was learning it and figuring it out. But Nanda Syahrasyad has outdone me easily in <a href="http://post.spmailtechnolo.com/f/a/OkJAGVpiNUMXyrnU7nLwvQ%7E%7E/AABEfgA%7E/RgRmz13gP0QdaHR0cHM6Ly93d3cubmFuLmZ5aS9zdmctcGF0aHNXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Understanding SVG Paths</a>.</p>
<p>The trick is understanding the commands. They are pretty understandable in the end, as it were. They are like “pick the pen up and move it here, then draw a line over to here” or “starting where you are, move the pen in this direction this far” or “draw a curve from here to there using these other two points as essentially gravitational poles.”</p>
<p>If you get into it, you’ll find yourselves (gasp) drawing your own shapes. I love Nanda’s opener:</p>
<img src="https://res.cloudinary.com/css-tricks/image/upload/c_scale,w_442,h_520,dpr_2/f_auto,q_auto/v1693006100/Screen-Capture-on-2023-08-25-at-16-27-53.gif?_i=AA" width="550">
<p>I think you’re kind of a next-level front-end developer if you’re building bending ass buttons like that.</p>
<hr>
<h3><a href="http://post.spmailtechnolo.com/f/a/6IiMSSL0wusOX0IAdTF2HA%7E%7E/AABEfgA%7E/RgRmz13gP0RbaHR0cHM6Ly93d3cuc3RlZmFuanVkaXMuY29tL3RvZGF5LWktbGVhcm5lZC9zdmdzLWhhdmUtYWRkaXRpb25hbC1wb2ludGVyLWV2ZW50cy1wcm9wZXJ0aWVzL1cDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">SVGs have additional pointer-events properties</a></h3>
<p>If you’re like me, you think of <code>pointer-events</code> in CSS as a thing you use to set <code>none</code> once in a while. Maybe you set some colored overlay <code>&lt;div&gt;</code> over something, but you don’t want it to actually eat up clicks, so you set <code>pointer-events: none</code> on it and those clicks will slide right through.</p>
<p>When it comes to SVG, though, <a href="http://post.spmailtechnolo.com/f/a/6IiMSSL0wusOX0IAdTF2HA%7E%7E/AABEfgA%7E/RgRmz13gP0RbaHR0cHM6Ly93d3cuc3RlZmFuanVkaXMuY29tL3RvZGF5LWktbGVhcm5lZC9zdmdzLWhhdmUtYWRkaXRpb25hbC1wb2ludGVyLWV2ZW50cy1wcm9wZXJ0aWVzL1cDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank">Stefan Judis has noted</a> some additional values for it that are specific to SVG like:</p>

<pre><code>.foo {
  pointer-events: visiblePainted;
}</code></pre>

<p><a href="http://post.spmailtechnolo.com/f/a/PwSNywbuhTrXnV76k2UF0Q%7E%7E/AABEfgA%7E/RgRmz13gP0QsaHR0cHM6Ly9jb2RlcGVuLmlvL01hcnRpam5DdXBwZW5zL3Blbi9NQmpxYk1XA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">The demo by Martijn Cuppens helps</a>. See number 4. Like ONLY the “painted” part is clickable there. I feel like that opens up some weird “click map” possibilities, so please send them to me if you do something weird.</p>

                                    </div>
                                  </mj-raw>
                                </div>
                              </td>
                            </tr>
                          </tbody>
                        </table>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>

            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div style="background:#333845;background-color:#333845;margin:0px auto;max-width:600px;">
      <table align="center" border="0" cellpadding="0" cellspacing="0" role="presentation" style="background:#333845;background-color:#333845;width:100%;">
        <tbody>
          <tr>
            <td style="direction:ltr;font-size:0px;padding:20px 0;text-align:center;">

              <div class="mj-column-per-100 mj-outlook-group-fix" style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                <table border="0" cellpadding="0" cellspacing="0" role="presentation" width="100%">
                  <tbody>
                    <tr>
                      <td style="vertical-align:top;padding:15px;">
                        <table border="0" cellpadding="0" cellspacing="0" role="presentation" style="" width="100%">
                          <tbody>
                            <tr>
                              <td align="left" style="font-size:0px;padding:10px 25px;word-break:break-word;">
                                <div style="font-family:Lato, system-ui, sans-serif;font-size:13px;line-height:1;text-align:left;color:#e3e4e8;">
                                  <mj-raw>
                                    <h2 class="pro-header">
                                      <a href="http://post.spmailtechnolo.com/f/a/CdJIqH5o4OTSNDz7wl8kgQ%7E%7E/AABEfgA%7E/RgRmz13gP0QXaHR0cHM6Ly9jb2RlcGVuLmlvL3Byby9XA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank"> CodePen PRO </a>
                                    </h2>
                                    <div class="pro-bar"></div>
                                    <div class="pro-content">
                                      <p>If you're ever in a situation where you're using CodePen to present something, you should try out <a href="http://post.spmailtechnolo.com/f/a/f5xTdP9t71Y5LIJvF7NteQ%7E%7E/AABEfgA%7E/RgRmz13gP0Q4aHR0cHM6Ly9ibG9nLmNvZGVwZW4uaW8vZG9jdW1lbnRhdGlvbi9wcmVzZW50YXRpb24tbW9kZS9XA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">Presentation Mode</a>. As in, share over Zoom where you have no idea the size of the screens of people watching. Or at a conference or classroom using an overhead projector. Or you're recording a YouTube video even!

</p><p>Presentation Mode allows you to hide the header, offering more screen real estate, and allows you quickly change (via options in the footer) visual things like the colors and how things are sized). From experience, it's often <em>very nice</em> to be able to flip to a "light" theme and jack up the font size on-the-fly.</p>
                                    </div>
                                  </mj-raw>
                                </div>
                              </td>
                            </tr>
                          </tbody>
                        </table>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>

            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div style="margin:0px auto;max-width:600px;">
      <table align="center" border="0" cellpadding="0" cellspacing="0" role="presentation" style="width:100%;">
        <tbody>
          <tr>
            <td style="direction:ltr;font-size:0px;padding:20px 0;text-align:center;">

              <div class="mj-column-per-100 mj-outlook-group-fix" style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                <table border="0" cellpadding="0" cellspacing="0" role="presentation" width="100%">
                  <tbody>
                    <tr>
                      <td style="vertical-align:top;padding:25px;">
                        <table border="0" cellpadding="0" cellspacing="0" role="presentation" style="" width="100%">
                          <tbody>
                            <tr>
                              <td align="left" style="font-size:0px;padding:10px 25px;word-break:break-word;">
                                <div style="font-family:Lato, system-ui, sans-serif;font-size:13px;line-height:1;text-align:left;color:#eeeeee;">
                                  <p class="subscription-details"> You can adjust your <a href="http://post.spmailtechnolo.com/f/a/OAo92upGMSYkQb_20uYSRA%7E%7E/AABEfgA%7E/RgRmz13gP0QpaHR0cHM6Ly9jb2RlcGVuLmlvL3NldHRpbmdzL25vdGlmaWNhdGlvbnNXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank"> email preferences </a> any time, or <a href="http://post.spmailtechnolo.com/f/a/aEuODOi06a2M_8xeNlCWDw%7E%7E/AABEfgA%7E/RgRmz13gP0S7aHR0cHM6Ly9jb2RlcGVuLmlvL3NldHRpbmdzL25vdGlmaWNhdGlvbnMvb3B0b3V0P191bj1pdTZ0UGtjJTJGQnl1JTJCZ0R1UXlORlpoNVFESTJqc2wlMkJ3S0pJUFNLZGpnZVZVbTVnakJEMXVDcCUyQjJ3RVViWmslMkJwTWhGWm80U3pvTUFjdGhKMFplNE5kSnMwaVV5YXRNY0p0OE5VRTVzRWJndkRRZGVIQ0V2aEtkQSUzRCUzRFcDc3BjQgpk5_TY7GTOfJCbUhAxNjY5OTc5ODJAcXEuY29tWAQAAAAA" rel="noopener" target="_blank"> instantly opt out </a> of emails of this kind. Need help with anything? Hit up <a href="http://post.spmailtechnolo.com/f/a/dS7GXAPZjFioisYP4q-Vjg%7E%7E/AABEfgA%7E/RgRmz13gP0QaaHR0cHM6Ly9jb2RlcGVuLmlvL3N1cHBvcnRXA3NwY0IKZOf02OxkznyQm1IQMTY2OTk3OTgyQHFxLmNvbVgEAAAAAA%7E%7E" rel="noopener" target="_blank">support</a>. </p>
                                </div>
                              </td>
                            </tr>
                          </tbody>
                        </table>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>

              <div class="footer-bar"></div>

            </td>
          </tr>
        </tbody>
      </table>
    </div>

  </div>"##;
    return x.to_string();
}