/// to define basic element of a policy
/// refer to the section 6, Security model, of the standard doc

struct User {
    name: String,
    // attributes: Vec<UserAttribute>,
}
struct UserAttribute {
    name: String,
    // attributes: Vec<UserAttribute>,
    // classes: Vec<PolicyClass>,
}
struct Object {
    name: String,
    // attributes: Vec<ObjectAttribute>,
    // classes: Vec<PolicyClass>,
}
struct ObjectAttribute {
    name: String,
    // attributes: Vec<ObjectAttribute>,
    // classes: Vec<PolicyClass>,
}

type Operation = String;

type PolicyClass = String;

struct PolicyElementSet {
    users: Vec<User>,
    user_attributes: Vec<UserAttribute>,
    objects: Vec<Object>,
    object_attributes: Vec<ObjectAttribute>,
    policy_classes: Vec<PolicyClass>,
}
enum Assignment {
    u2ua(User, UserAttribute),
    ua2ua(UserAttribute, UserAttribute),
    oa2oa(),
    ua2pc(),
    oa2pc(),
}
type AccessRightSet = Vec<Operation>;
enum Association {
    ua2oa(),
    ua2ua(),
}
type Prohibition = Association;
type Pattern = String;
type Response = String;
struct Obligation(User, Pattern, Response);
struct PolicyRelationSet {
    assignments: Vec<Assignment>,
    associations: Vec<Association>,
    prohibitions: Vec<Prohibition>,
    obligations: Vec<Obligation>,
}

struct PolicySet {
    pe: PolicyElementSet,
    ar: AccessRightSet,
    pr: PolicyRelationSet,
}
trait PolicySetFn {
    fn verify(&self) -> bool;
}

impl PolicySetFn for PolicySet {
    fn verify(&self) -> bool {
        true
    }
}
