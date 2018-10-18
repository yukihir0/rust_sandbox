extern crate diesel_relation_sample;

extern crate diesel;

use self::diesel_relation_sample::*;

use diesel::prelude::*;
use diesel::result::Error;

fn main() {
    let conn = establish_connection();

    initialize_database(&conn);

    // inintialize data assert
    conn.test_transaction::<_, Error, _>(|| {
        let user1 = select_user(&conn, 1);
        assert_eq!(user1.id, 1);
        assert_eq!(user1.name, "hoge");

        let user2 = select_user(&conn, 2);
        assert_eq!(user2.id, 2);
        assert_eq!(user2.name, "fuga");

        let post1 = select_post(&conn, 1);
        assert_eq!(post1.id, 1);
        assert_eq!(post1.title, "post01 by hoge");
        assert_eq!(post1.user_id, 1);

        let post2 = select_post(&conn, 2);
        assert_eq!(post2.id, 2);
        assert_eq!(post2.title, "post02 by hoge");
        assert_eq!(post2.user_id, 1);

        let post3 = select_post(&conn, 3);
        assert_eq!(post3.id, 3);
        assert_eq!(post3.title, "post03 by fuga");
        assert_eq!(post3.user_id, 2);

        let post4 = select_post(&conn, 4);
        assert_eq!(post4.id, 4);
        assert_eq!(post4.title, "post04 by fuga");
        assert_eq!(post4.user_id, 2);

        let tag1 = select_tag(&conn, 1);
        assert_eq!(tag1.id, 1);
        assert_eq!(tag1.label, "tag01");

        let tag2 = select_tag(&conn, 2);
        assert_eq!(tag2.id, 2);
        assert_eq!(tag2.label, "tag02");

        let tag3 = select_tag(&conn, 3);
        assert_eq!(tag3.id, 3);
        assert_eq!(tag3.label, "tag03");

        let post_tag1 = select_post_tag(&conn, 1);
        assert_eq!(post_tag1.id, 1);
        assert_eq!(post_tag1.post_id, 1);
        assert_eq!(post_tag1.tag_id, 1);

        let post_tag2 = select_post_tag(&conn, 2);
        assert_eq!(post_tag2.id, 2);
        assert_eq!(post_tag2.post_id, 2);
        assert_eq!(post_tag2.tag_id, 1);

        let post_tag3 = select_post_tag(&conn, 3);
        assert_eq!(post_tag3.id, 3);
        assert_eq!(post_tag3.post_id, 2);
        assert_eq!(post_tag3.tag_id, 2);

        let post_tag4 = select_post_tag(&conn, 4);
        assert_eq!(post_tag4.id, 4);
        assert_eq!(post_tag4.post_id, 3);
        assert_eq!(post_tag4.tag_id, 3);

        let post_tag5 = select_post_tag(&conn, 5);
        assert_eq!(post_tag5.id, 5);
        assert_eq!(post_tag5.post_id, 4);
        assert_eq!(post_tag5.tag_id, 2);

        let post_tag6 = select_post_tag(&conn, 6);
        assert_eq!(post_tag6.id, 6);
        assert_eq!(post_tag6.post_id, 4);
        assert_eq!(post_tag6.tag_id, 3);

        Ok(())
    });
   
    // user -> posts assert
    conn.test_transaction::<_, Error, _>(|| {
        let posts1 = select_posts_by_user(&conn, 1);
        assert_eq!(posts1.len(), 2);

        assert_eq!(posts1[0].id, 1);
        assert_eq!(posts1[0].title, "post01 by hoge");
        assert_eq!(posts1[0].user_id, 1);

        assert_eq!(posts1[1].id, 2);
        assert_eq!(posts1[1].title, "post02 by hoge");
        assert_eq!(posts1[1].user_id, 1);

        let posts2 = select_posts_by_user(&conn, 2);
        assert_eq!(posts2.len(), 2);
        
        assert_eq!(posts2[0].id, 3);
        assert_eq!(posts2[0].title, "post03 by fuga");
        assert_eq!(posts2[0].user_id, 2);

        assert_eq!(posts2[1].id, 4);
        assert_eq!(posts2[1].title, "post04 by fuga");
        assert_eq!(posts2[1].user_id, 2);

        Ok(())
    });

    // tag -> post_tag -> post
    conn.test_transaction::<_, Error, _>(|| {
        let posts1 = select_posts_by_tag(&conn, 1);
        assert_eq!(posts1.len(), 2);

        assert_eq!(posts1[0].id, 1);
        assert_eq!(posts1[0].title, "post01 by hoge");
        assert_eq!(posts1[0].user_id, 1);

        assert_eq!(posts1[1].id, 2);
        assert_eq!(posts1[1].title, "post02 by hoge");
        assert_eq!(posts1[1].user_id, 1);

        let posts2 = select_posts_by_tag(&conn, 2);
        assert_eq!(posts2.len(), 2);

        assert_eq!(posts2[0].id, 2);
        assert_eq!(posts2[0].title, "post02 by hoge");
        assert_eq!(posts2[0].user_id, 1);

        assert_eq!(posts2[1].id, 4);
        assert_eq!(posts2[1].title, "post04 by fuga");
        assert_eq!(posts2[1].user_id, 2);

        let posts3 = select_posts_by_tag(&conn, 3);
        assert_eq!(posts3.len(), 2);

        assert_eq!(posts3[0].id, 3);
        assert_eq!(posts3[0].title, "post03 by fuga");
        assert_eq!(posts3[0].user_id, 2);

        assert_eq!(posts3[1].id, 4);
        assert_eq!(posts3[1].title, "post04 by fuga");
        assert_eq!(posts3[1].user_id, 2);

        Ok(())
    });


    // post -> post_tag -> tag
    conn.test_transaction::<_, Error, _>(|| {
        let tags1 = select_tags_by_post(&conn, 1);
        assert_eq!(tags1.len(), 1);

        assert_eq!(tags1[0].id, 1);
        assert_eq!(tags1[0].label, "tag01");

        let tags2 = select_tags_by_post(&conn, 2);
        assert_eq!(tags2.len(), 2);

        assert_eq!(tags2[0].id, 1);
        assert_eq!(tags2[0].label, "tag01");

        assert_eq!(tags2[1].id, 2);
        assert_eq!(tags2[1].label, "tag02");

        let tags3 = select_tags_by_post(&conn, 3);
        assert_eq!(tags3.len(), 1);

        assert_eq!(tags3[0].id, 3);
        assert_eq!(tags3[0].label, "tag03");

        let tags4 = select_tags_by_post(&conn, 4);
        assert_eq!(tags4.len(), 2);

        assert_eq!(tags4[0].id, 2);
        assert_eq!(tags4[0].label, "tag02");

        assert_eq!(tags4[1].id, 3);
        assert_eq!(tags4[1].label, "tag03");

        Ok(())
    });
}
