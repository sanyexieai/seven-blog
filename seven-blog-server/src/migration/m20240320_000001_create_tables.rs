use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建用户信息表
        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserInfo::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(UserInfo::Email).string().null())
                    .col(ColumnDef::new(UserInfo::Nickname).string().not_null())
                    .col(ColumnDef::new(UserInfo::Avatar).string().not_null())
                    .col(ColumnDef::new(UserInfo::Intro).string().not_null())
                    .col(ColumnDef::new(UserInfo::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(UserInfo::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建用户表
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::UserInfoId).integer().not_null())
                    .col(ColumnDef::new(User::LoginType).integer().not_null().default(0))
                    .col(ColumnDef::new(User::IpAddress).string().null())
                    .col(ColumnDef::new(User::LastLoginTime).timestamp().null())
                    .col(ColumnDef::new(User::IsDisable).boolean().not_null().default(false))
                    .col(ColumnDef::new(User::IsSuper).boolean().not_null().default(false))
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_user_info")
                            .from(User::Table, User::UserInfoId)
                            .to(UserInfo::Table, UserInfo::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建角色表
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Role::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Role::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Role::Description).string().null())
                    .to_owned(),
            )
            .await?;

        // 创建用户角色关联表
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRole::UserId).integer().not_null())
                    .col(ColumnDef::new(UserRole::RoleId).integer().not_null())
                    .primary_key(Index::create().name("pk_user_role").col(UserRole::UserId).col(UserRole::RoleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role_user")
                            .from(UserRole::Table, UserRole::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role_role")
                            .from(UserRole::Table, UserRole::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建分类表
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Category::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Category::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Category::Description).string().null())
                    .to_owned(),
            )
            .await?;

        // 创建标签表
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tag::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Tag::Name).string().not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        // 创建文章表
        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Article::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Article::Title).string().not_null())
                    .col(ColumnDef::new(Article::Content).text().not_null())
                    .col(ColumnDef::new(Article::Summary).string().not_null())
                    .col(ColumnDef::new(Article::Cover).string().null())
                    .col(ColumnDef::new(Article::ViewCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Article::LikeCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Article::CommentCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Article::IsTop).boolean().not_null().default(false))
                    .col(ColumnDef::new(Article::IsRecommend).boolean().not_null().default(false))
                    .col(ColumnDef::new(Article::Status).integer().not_null().default(0))
                    .col(ColumnDef::new(Article::UserId).integer().not_null())
                    .col(ColumnDef::new(Article::CategoryId).integer().null())
                    .col(ColumnDef::new(Article::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Article::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_article_user")
                            .from(Article::Table, Article::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_article_category")
                            .from(Article::Table, Article::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建文章标签关联表
        manager
            .create_table(
                Table::create()
                    .table(ArticleTag::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ArticleTag::ArticleId).integer().not_null())
                    .col(ColumnDef::new(ArticleTag::TagId).integer().not_null())
                    .primary_key(Index::create().name("pk_article_tag").col(ArticleTag::ArticleId).col(ArticleTag::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_article_tag_article")
                            .from(ArticleTag::Table, ArticleTag::ArticleId)
                            .to(Article::Table, Article::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_article_tag_tag")
                            .from(ArticleTag::Table, ArticleTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建评论表
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comment::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Comment::Content).text().not_null())
                    .col(ColumnDef::new(Comment::UserId).integer().not_null())
                    .col(ColumnDef::new(Comment::ArticleId).integer().not_null())
                    .col(ColumnDef::new(Comment::ParentId).integer().null())
                    .col(ColumnDef::new(Comment::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Comment::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_user")
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_article")
                            .from(Comment::Table, Comment::ArticleId)
                            .to(Article::Table, Article::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_parent")
                            .from(Comment::Table, Comment::ParentId)
                            .to(Comment::Table, Comment::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 插入默认角色
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Role::Table)
                    .columns([Role::Name, Role::Description])
                    .values_panic(["admin".into(), "管理员".into()])
                    .values_panic(["user".into(), "普通用户".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除所有表（注意顺序，先删除有外键约束的表）
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ArticleTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserInfo::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum UserInfo {
    Table,
    Id,
    Email,
    Nickname,
    Avatar,
    Intro,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    PasswordHash,
    UserInfoId,
    LoginType,
    IpAddress,
    LastLoginTime,
    IsDisable,
    IsSuper,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Role {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
enum UserRole {
    Table,
    UserId,
    RoleId,
}

#[derive(Iden)]
enum Category {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
enum Tag {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Article {
    Table,
    Id,
    Title,
    Content,
    Summary,
    Cover,
    ViewCount,
    LikeCount,
    CommentCount,
    IsTop,
    IsRecommend,
    Status,
    UserId,
    CategoryId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum ArticleTag {
    Table,
    ArticleId,
    TagId,
}

#[derive(Iden)]
enum Comment {
    Table,
    Id,
    Content,
    UserId,
    ArticleId,
    ParentId,
    CreatedAt,
    UpdatedAt,
} 