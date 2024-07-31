# rss-monitor

```
rss-monitor 
    rss-watcher 
        source 
            source 
            watcher 
            sink 
        watcher 
            task 
                fetch feeds 
                translate 
                collect 
                cahce 
                    Redis 
                post to sink 
        sink 
            discord 
            custom 
        config 
            base 
            format 
                markdown support 
                embedded format 
                视频/图片(参考其他机器人)
        logger 
        filter 
            基于正则表达式 
            基于关键词 
            替换 
            评论屏蔽(sina、bilibili) 
    futures 
        dashboard 
        支持discord robot配置 
        schedule 
                不重要的feed每日汇总推送 
                重要的邮件通知 
                监控数据/报表每日推送 
                发生错误邮件通知 
        nitter支持 
        eddit thread支持(外部组件)
```